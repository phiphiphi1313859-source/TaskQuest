use crate::character::Class;

/// ASCII avatar for a character at a specific level
pub struct Avatar {
    pub art: &'static str,
    pub min_level: u32,
}

impl Class {
    /// Get the appropriate avatar for this class at the given level
    pub fn get_avatar(&self, level: u32) -> &'static str {
        let avatars = match self {
            Class::Rogue => ROGUE_AVATARS,
            Class::Ranger => RANGER_AVATARS,
            Class::Warrior => WARRIOR_AVATARS,
            Class::Paladin => PALADIN_AVATARS,
            Class::Monk => MONK_AVATARS,
        };

        // Find the appropriate avatar for this level
        avatars
            .iter()
            .rev() // Start from highest level
            .find(|avatar| level >= avatar.min_level)
            .map(|avatar| avatar.art)
            .unwrap_or(avatars[0].art)
    }
}

// ============================================================================
// ROGUE AVATARS - DEX/CHA - Stealthy, agile, cunning
// ============================================================================

const ROGUE_AVATARS: &[Avatar] = &[
    // Level 1-9: Novice Rogue
    Avatar {
        min_level: 1,
        art: r#"
    ___
   /   \
  | o o |
   \ - /
    |=|
   /| |\
  / | | \
    | |
   /   \
"#
    },
    // Level 10-19: Skilled Thief
    Avatar {
        min_level: 10,
        art: r#"
     ___
    /   \
   | * * |
    \ ^ /
    _|=|_
   /~| |~\
  / ~| |~ \
    /| |\
   / | | \
"#
    },
    // Level 20-29: Master Assassin
    Avatar {
        min_level: 20,
        art: r#"
      ___
    _/   \_
   | ** ** |
    \  ^  /
   __|=|=|__
  /~~| | |~~\
 |~~~| | |~~~|
    /|   |\
   / |   | \
  /_  \_/  _\
"#
    },
    // Level 30-39: Shadow Dancer
    Avatar {
        min_level: 30,
        art: r#"
       ___
     _/   \_
    |.**.**.|
     \ ^~^ /
   __|=|=|=|__
  /~~~| | |~~~\
 |~~~~| | |~~~~|
  \  /|   |\  /
   \/ |   | \/
  /~\_  \_/ _/~\
"#
    },
    // Level 40+: Legendary Phantom
    Avatar {
        min_level: 40,
        art: r#"
        ___
      _/   \_
    _|.**.**.| _
     |\  ^  /|
   __|=|=*=|=|__
  /~~~|~|~|~|~~~\
 |~~~~|~|~|~|~~~~|
  \ ~/| | | |\~ /
   \/~| | | |~\/
  /~~~\_  \_/~~~\
 ~~~~~~~~~~~~~~~~
"#
    },
];

// ============================================================================
// RANGER AVATARS - DEX/WIS - Nature-attuned, archer, tracker
// ============================================================================

const RANGER_AVATARS: &[Avatar] = &[
    // Level 1-9: Novice Ranger
    Avatar {
        min_level: 1,
        art: r#"
    ___
   /   \
  | o o |
   \ - /
    |^|
   /|)|\
  / | | \
    | |
   / ^ \
"#
    },
    // Level 10-19: Woodland Scout
    Avatar {
        min_level: 10,
        art: r#"
     ___
    /   \
   | * * |
    \ - /
   _|^|^|_
  /)| | |(\
 / )| | |( \
   /| | |\
  / | | | \
 /_  ^   ^ \
"#
    },
    // Level 20-29: Master Hunter
    Avatar {
        min_level: 20,
        art: r#"
      ___
    _/^^^\_
   | ** ** |
    \ --- /
   __|^|^|__
  /))| | |((\
 ()))| | |((()
  \ /| | |\ /
   V | | | V
  /^\_  \_/^\
"#
    },
    // Level 30-39: Beast Master
    Avatar {
        min_level: 30,
        art: r#"
   ~   ___   ~
  ~~_/^^^\_~~
   |~** **~|
    \ --- /
  ___|^|^|___
 /)))| | |(((\
())))|~|~|(((()
 \ ~/| | |\ ~/
  \V | | | V/
  /^~\_  \_/~^\
 ~~~~~~~~~~~~~~~
"#
    },
    // Level 40+: Legendary Pathfinder
    Avatar {
        min_level: 40,
        art: r#"
  ~~~ _____ ~~~
  ~~~/^^^^^\~~~
  ~|~**~**~|~
   |\ --- /|
  ___|^*^|___
 /))))|~|((((\
()))))|~|((((()
 \~~/|~|~|\~~/
  \V~| | |~V/
 /^~~\_  \_/~~^\
~~~  ~~~ ~~~  ~~~
"#
    },
];

// ============================================================================
// WARRIOR AVATARS - STR/CON - Strong, armored, weapon master
// ============================================================================

const WARRIOR_AVATARS: &[Avatar] = &[
    // Level 1-9: Novice Fighter
    Avatar {
        min_level: 1,
        art: r#"
    ___
   /   \
  | o o |
   \ - /
   [|=|]
   /| |\
  / | | \
    |I|
   / I \
"#
    },
    // Level 10-19: Veteran Soldier
    Avatar {
        min_level: 10,
        art: r#"
     ___
    /   \
   | O O |
    \ = /
   [|=|=|]
  [[| | |]]
 / [| | |] \
   /|III|\
  / |III| \
    /III\
"#
    },
    // Level 20-29: Battle Champion
    Avatar {
        min_level: 20,
        art: r#"
      ___
    _/   \_
   |[O=O]|
    \\=/=/
  _[|=|=|]_
 [[=| | |=]]
[[[=| | |=]]]
  \[|III|]/
  /[|III|]\
   [/III\]
"#
    },
    // Level 30-39: War Master
    Avatar {
        min_level: 30,
        art: r#"
      _____
    _/[===]\_
   |[[O=O]]|
    |\\=/=/|
  _[|=|=|=|]_
 [[==| | |==]]
[[[==| | |==]]]
 \[[|III|]]/
  \[|III|]/
  [[/III\]]
   -------
"#
    },
    // Level 40+: Legendary Warlord
    Avatar {
        min_level: 40,
        art: r#"
     _______
   _/[=====]\_
  |[[[O=O]]]|
   |\\\==/=/|
  _[|=|=*=|]_
 [[===| |===]]
[[[[==| |==]]]]
 \[[[III]]]/
  \[[III]]/
  [[[III]]]
  ---------
  =========
"#
    },
];

// ============================================================================
// PALADIN AVATARS - STR/CHA - Holy warrior, righteous defender
// ============================================================================

const PALADIN_AVATARS: &[Avatar] = &[
    // Level 1-9: Novice Paladin
    Avatar {
        min_level: 1,
        art: r#"
    ___
   /   \
  | o o |
   \ - /
   {|+|}
   /| |\
  / | | \
    |I|
   / + \
"#
    },
    // Level 10-19: Holy Knight
    Avatar {
        min_level: 10,
        art: r#"
     ___
    / + \
   | O O |
    \ = /
   {|+|+|}
  {[| | |]}
 / {| | |} \
   /|III|\
  / |I+I| \
    / + \
"#
    },
    // Level 20-29: Divine Champion
    Avatar {
        min_level: 20,
        art: r#"
      _+_
    _/ + \_
   |{O+O}|
    \\+/+/
  _{|+|+|}_
 {[+| | |+]}
{[[+| | |+]]}
  \{|III|}/
  /{|I+I|}\
   {/ + \}
"#
    },
    // Level 30-39: Crusader Saint
    Avatar {
        min_level: 30,
        art: r#"
      __+__
    _/{+++}\_
   |{{O+O}}|
    |\\+/+/|
  _{|+|+|+|}_
 {[++| | |++]}
{[[++| | |++]]}
 \{{|III|}}/
  \{|I+I|}/
  {{/ + \}}
   -------
"#
    },
    // Level 40+: Legendary Lightbringer
    Avatar {
        min_level: 40,
        art: r#"
    *  +  *
   _/{+++++}\_
  |{{{O+O}}}|
   |\\\+/+/|
  _{|+|+*+|}_
 {[+++| |+++]}
{{[[++| |++]]}}
 \{{{III}}}/
  \{{I+I}}/
  {{{/ + \}}}
  -----------
 * ======= *
"#
    },
];

// ============================================================================
// MONK AVATARS - DEX/WIS - Martial artist, spiritual discipline
// ============================================================================

const MONK_AVATARS: &[Avatar] = &[
    // Level 1-9: Novice Monk
    Avatar {
        min_level: 1,
        art: r#"
    ___
   /   \
  | - - |
   \ o /
    |.|
   /| |\
  / | | \
    | |
   / \ /
"#
    },
    // Level 10-19: Adept Disciple
    Avatar {
        min_level: 10,
        art: r#"
     ___
    / . \
   | = = |
    \ o /
   _|.|.|_
  /.| | |.\
 / .| | |. \
   /| | |\
  / \   / \
    \   /
"#
    },
    // Level 20-29: Master of Forms
    Avatar {
        min_level: 20,
        art: r#"
      ___
    _/. .\._
   | .=.=. |
    \ \o/ /
   __|.|.|__
  /..| | |..\
 /...| | |...\
  \ /| | |\ /
   X | | | X
  /.\   \./.\
"#
    },
    // Level 30-39: Enlightened One
    Avatar {
        min_level: 30,
        art: r#"
   .   ___   .
  .._/. .\._..
   |..=.=..|
    |\\o//|
  __|.|.|.|__
 /...| | |...\
/....| | |....\
 \ ./| | |\ ./
  \X | | | X/
  /..\   \./..\
 ..............
"#
    },
    // Level 40+: Legendary Grandmaster
    Avatar {
        min_level: 40,
        art: r#"
  . . _____ . .
  .../.....\...
  .|..=.=..|.
   |\\\o///|
  __|.|.*.|__
 /....| |....\
/.....| |.....\
 \.../|.|\.../
  \X.| | |.X/
 /....\  \.....\
. . . . .. . . .
 ...............
"#
    },
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_avatar_progression() {
        let warrior = Class::Warrior;

        // Should get different avatars at different levels
        let avatar1 = warrior.get_avatar(1);
        let avatar10 = warrior.get_avatar(10);
        let avatar20 = warrior.get_avatar(20);
        let avatar30 = warrior.get_avatar(30);
        let avatar40 = warrior.get_avatar(40);

        // All should be different
        assert_ne!(avatar1, avatar10);
        assert_ne!(avatar10, avatar20);
        assert_ne!(avatar20, avatar30);
        assert_ne!(avatar30, avatar40);
    }

    #[test]
    fn test_all_classes_have_avatars() {
        let classes = [
            Class::Rogue,
            Class::Ranger,
            Class::Warrior,
            Class::Paladin,
            Class::Monk,
        ];

        for class in &classes {
            let avatar = class.get_avatar(1);
            assert!(!avatar.is_empty(), "{:?} should have avatar", class);
        }
    }

    #[test]
    fn test_same_tier_same_avatar() {
        let rogue = Class::Rogue;

        // Same tier should give same avatar
        assert_eq!(rogue.get_avatar(5), rogue.get_avatar(9));
        assert_eq!(rogue.get_avatar(10), rogue.get_avatar(15));
        assert_eq!(rogue.get_avatar(20), rogue.get_avatar(25));
    }
}
