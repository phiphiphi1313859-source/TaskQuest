use anyhow::{Context, Result};
use git2::{Repository, Signature, IndexAddOption, StatusOptions};
use std::path::Path;
use colored::Colorize;

pub struct GitSync;

impl GitSync {
    /// Initialize a git repository in the data directory
    pub fn init(data_dir: &Path, remote_url: Option<String>) -> Result<()> {
        println!("Initializing git repository in {}...", data_dir.display());

        // Check if already initialized
        if data_dir.join(".git").exists() {
            println!("{}", "✓ Git repository already initialized".green());
            return Ok(());
        }

        // Initialize repository
        let repo = Repository::init(data_dir)
            .context("Failed to initialize git repository")?;

        // Create .gitignore
        let gitignore_path = data_dir.join(".gitignore");
        std::fs::write(&gitignore_path, "*.bak\n*.tmp\n.sync-conflict-*\n")
            .context("Failed to create .gitignore")?;

        // Add all JSON files
        let mut index = repo.index()?;
        index.add_all(["*.json"].iter(), IndexAddOption::DEFAULT, None)?;
        index.add_path(Path::new(".gitignore"))?;
        index.write()?;

        // Create initial commit
        let tree_id = index.write_tree()?;
        let tree = repo.find_tree(tree_id)?;
        let sig = Signature::now("TaskQuest", "noreply@taskquest.local")?;

        repo.commit(
            Some("HEAD"),
            &sig,
            &sig,
            "Initial TaskQuest data",
            &tree,
            &[],
        )?;

        println!("{}", "✓ Git repository initialized".green());

        // Add remote if provided
        if let Some(url) = remote_url {
            repo.remote("origin", &url)
                .context("Failed to add remote")?;
            println!("{}", format!("✓ Added remote: {}", url).green());
            println!();
            println!("Next steps:");
            println!("  1. Push to remote: taskquest sync push");
            println!("  2. Or configure your GitHub/GitLab repository");
        } else {
            println!();
            println!("Repository initialized locally. To sync with a remote:");
            println!("  git -C {} remote add origin <your-repo-url>", data_dir.display());
            println!("  taskquest sync push");
        }

        Ok(())
    }

    /// Commit current changes
    pub fn commit(data_dir: &Path, message: &str) -> Result<()> {
        let repo = Repository::open(data_dir)
            .context("Not a git repository. Run 'taskquest sync init' first")?;

        // Check for changes
        let mut status_opts = StatusOptions::new();
        status_opts.include_untracked(true);
        let statuses = repo.statuses(Some(&mut status_opts))?;

        if statuses.is_empty() {
            println!("{}", "No changes to commit".yellow());
            return Ok(());
        }

        // Stage all changes
        let mut index = repo.index()?;
        index.add_all(["*.json"].iter(), IndexAddOption::DEFAULT, None)?;
        index.write()?;

        // Create commit
        let tree_id = index.write_tree()?;
        let tree = repo.find_tree(tree_id)?;
        let sig = Signature::now("TaskQuest", "noreply@taskquest.local")?;
        let parent_commit = repo.head()?.peel_to_commit()?;

        repo.commit(
            Some("HEAD"),
            &sig,
            &sig,
            message,
            &tree,
            &[&parent_commit],
        )?;

        println!("{}", format!("✓ Committed: {}", message).green());
        Ok(())
    }

    /// Push changes to remote
    pub fn push(data_dir: &Path) -> Result<()> {
        let repo = Repository::open(data_dir)
            .context("Not a git repository. Run 'taskquest sync init' first")?;

        // Commit any pending changes first
        Self::commit(data_dir, &format!("TaskQuest update - {}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S")))?;

        // Check if remote exists
        let remote_name = "origin";
        let mut remote = repo.find_remote(remote_name)
            .context("No remote configured. Add a remote with: git -C ~/.taskquest remote add origin <url>")?;

        println!("Pushing to {}...", remote.url().unwrap_or("unknown"));

        // Push to remote
        remote.push(
            &["refs/heads/main:refs/heads/main"],
            None,
        ).context("Failed to push. You may need to configure authentication (SSH keys or credentials)")?;

        println!("{}", "✓ Pushed successfully".green());
        Ok(())
    }

    /// Pull changes from remote
    pub fn pull(data_dir: &Path) -> Result<()> {
        let repo = Repository::open(data_dir)
            .context("Not a git repository. Run 'taskquest sync init' first")?;

        println!("Pulling from remote...");

        // Fetch from remote
        let mut remote = repo.find_remote("origin")
            .context("No remote configured")?;
        remote.fetch(&["main"], None, None)?;

        // Get the fetch head
        let fetch_head = repo.find_reference("FETCH_HEAD")?;
        let fetch_commit = repo.reference_to_annotated_commit(&fetch_head)?;

        // Merge
        let analysis = repo.merge_analysis(&[&fetch_commit])?;

        if analysis.0.is_up_to_date() {
            println!("{}", "✓ Already up to date".green());
            return Ok(());
        }

        if analysis.0.is_fast_forward() {
            // Fast-forward merge
            let refname = "refs/heads/main";
            let mut reference = repo.find_reference(refname)?;
            reference.set_target(fetch_commit.id(), "Fast-forward")?;
            repo.set_head(refname)?;
            repo.checkout_head(Some(git2::build::CheckoutBuilder::default().force()))?;

            println!("{}", "✓ Fast-forwarded to latest".green());
            Ok(())
        } else {
            // Need to merge - this is more complex
            println!("{}", "⚠️  Manual merge required".yellow());
            println!("Changes on both local and remote. To merge:");
            println!("  cd ~/.taskquest");
            println!("  git merge FETCH_HEAD");
            println!("  # Resolve any conflicts");
            println!("  taskquest sync push");

            anyhow::bail!("Merge required - please resolve manually")
        }
    }

    /// Show sync status
    pub fn status(data_dir: &Path) -> Result<()> {
        let repo = Repository::open(data_dir)
            .context("Not a git repository. Run 'taskquest sync init' first")?;

        println!();
        println!("{}", "╔════════════════════════════════════════╗".cyan());
        println!("{}", "║          SYNC STATUS                   ║".cyan().bold());
        println!("{}", "╠════════════════════════════════════════╣".cyan());

        // Check for changes
        let mut status_opts = StatusOptions::new();
        status_opts.include_untracked(true);
        let statuses = repo.statuses(Some(&mut status_opts))?;

        if statuses.is_empty() {
            println!("║ Working tree: {} ║", "clean".green().bold());
        } else {
            println!("║ Working tree: {} ║", "modified".yellow().bold());
            for entry in statuses.iter() {
                let path = entry.path().unwrap_or("unknown");
                println!("║   Modified: {}                        ║", path);
            }
        }

        // Check remote
        match repo.find_remote("origin") {
            Ok(remote) => {
                if let Some(url) = remote.url() {
                    println!("║ Remote: {}                             ║", url);
                }
            }
            Err(_) => {
                println!("║ Remote: {} ║", "not configured".red());
            }
        }

        // Show last commit
        if let Ok(head) = repo.head() {
            if let Ok(commit) = head.peel_to_commit() {
                let msg = commit.message().unwrap_or("").lines().next().unwrap_or("");
                let time = commit.time();
                let datetime = chrono::NaiveDateTime::from_timestamp_opt(time.seconds(), 0)
                    .unwrap_or_default();
                println!("║ Last commit: {}                        ║", datetime.format("%Y-%m-%d %H:%M"));
                println!("║ Message: {}                            ║", msg);
            }
        }

        println!("{}", "╚════════════════════════════════════════╝".cyan());
        println!();

        Ok(())
    }

    /// Show commit history
    pub fn history(data_dir: &Path, limit: usize) -> Result<()> {
        let repo = Repository::open(data_dir)
            .context("Not a git repository. Run 'taskquest sync init' first")?;

        println!();
        println!("{}", "╔════════════════════════════════════════════════════════════╗".yellow());
        println!("{}", "║                  SYNC HISTORY                              ║".yellow().bold());
        println!("{}", "╚════════════════════════════════════════════════════════════╝".yellow());
        println!();

        let mut revwalk = repo.revwalk()?;
        revwalk.push_head()?;

        for (i, oid) in revwalk.enumerate() {
            if i >= limit {
                break;
            }

            let oid = oid?;
            let commit = repo.find_commit(oid)?;

            let time = commit.time();
            let datetime = chrono::NaiveDateTime::from_timestamp_opt(time.seconds(), 0)
                .unwrap_or_default();

            let msg = commit.message().unwrap_or("").lines().next().unwrap_or("");
            let short_id = &oid.to_string()[..7];

            println!("{} {} {}",
                short_id.cyan().bold(),
                datetime.format("%Y-%m-%d %H:%M").to_string().yellow(),
                msg
            );
        }

        println!();
        Ok(())
    }
}
