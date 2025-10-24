use clap::{Parser, Subcommand};
use colored::*;
use rand::seq::SliceRandom;
use inquire::{Text, Confirm, Select};

#[derive(Parser)]
#[command(name = "snack")]
#[command(about = "A transition snack CLI tool to help ease transitions and overcome procrastination", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Start the guided 5-step procrastination-breaking process
    Guide,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Guide) => {
            run_guide_mode();
        }
        None => {
            run_quick_snack();
        }
    }
}

fn run_quick_snack() {
    let snacks = vec![
        SnackItem {
            art: r#"
       /\_/\
      ( ^.^ ) ~meow~
       > 🐾 <
      /|   |\
     (_|   |_)
  "#,
            message: "You're transitioning. That's okay.",
            color: "cyan",
        },
        SnackItem {
            art: r#"
        ✨
       ╱|、
      (˚ˎ 。7
       |、˜〵
      じしˍ,)ノ
  "#,
            message: "Taking a moment is productive.",
            color: "yellow",
        },
        SnackItem {
            art: r#"
         🌸
        ╱|、
       (˚ˎ。7
        |、˜〵
       じしˍ,)ノ 🌿
  "#,
            message: "Growth happens in small steps.",
            color: "green",
        },
        SnackItem {
            art: r#"
       ☕
      ( (
       ) )
    ┌─────┐
    │~~~~~│
    └─────┘
     \ | /
      \_/
  "#,
            message: "Breathe. You've got this.",
            color: "magenta",
        },
        SnackItem {
            art: r#"
        _____     ____
       /      \  |  o |
      |        |/ ___\|
      |_________/
      |_|_| |_|_|
  "#,
            message: "Like a turtle, slow and steady wins.",
            color: "green",
        },
        SnackItem {
            art: r#"
       ⭐
      ╱|、
     (๑•̀ㅁ•́๑)✧
      /    \
     💪    💪
  "#,
            message: "Every journey starts with a single step.",
            color: "blue",
        },
        SnackItem {
            art: r#"
         🌙
        ★ ∧＿∧
       ★  (｡･ω･｡)ﾉ
        ／ づ💤
  "#,
            message: "Rest is part of progress too.",
            color: "magenta",
        },
        SnackItem {
            art: r#"
       ╔═══╗ ♪
       ║███║ ♫
       ║ (●)║♫
       ╚═══╝♪
  "#,
            message: "Let's jam through this together.",
            color: "yellow",
        },
        SnackItem {
            art: r#"
      ∧＿∧
     (｡･ω･｡)つ━☆・*。
     ⊂　 ノ 　　　・゜+.
      しーＪ　　　°。+ *´¨)
  "#,
            message: "You're doing magic just by trying.",
            color: "cyan",
        },
        SnackItem {
            art: r#"
       🎈
      ＼(＾▽＾)／
        |   |
       ノ   ヽ
  "#,
            message: "Celebrate small wins!",
            color: "yellow",
        },
        SnackItem {
            art: r#"
     ☆.。.:*・°☆.。.:*・°☆
       づ ◕‿◕ )づ
     ☆.。.:*・°☆.。.:*・°☆
  "#,
            message: "You've got this. I believe in you.",
            color: "magenta",
        },
        SnackItem {
            art: r#"
       🌈
      ╱|、
     (˘ω˘ )
      |、  づ
      じしˍ)   \(ᵔᵕᵔ)/
  "#,
            message: "Two is better than one. You're not alone.",
            color: "cyan",
        },
    ];

    let mut rng = rand::thread_rng();
    let snack = snacks.choose(&mut rng).unwrap();

    println!();
    match snack.color {
        "cyan" => println!("{}", snack.art.cyan()),
        "yellow" => println!("{}", snack.art.yellow()),
        "green" => println!("{}", snack.art.green()),
        "magenta" => println!("{}", snack.art.magenta()),
        "blue" => println!("{}", snack.art.blue()),
        _ => println!("{}", snack.art),
    }

    println!("{}", snack.message.bold());
    println!();
}

struct SnackItem {
    art: &'static str,
    message: &'static str,
    color: &'static str,
}

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}

fn run_guide_mode() {
    clear_screen();
    println!();
    println!("{}", "🎯 Welcome to the Transition Guide".cyan().bold());
    println!();

    // Step 1: Acknowledge
    if !step_1_acknowledge() {
        return;
    }

    // Step 2: Examine feelings
    if !step_2_examine() {
        return;
    }

    // Step 3: Mood boost
    if !step_3_mood_boost() {
        return;
    }

    // Step 4: Break down task
    let tiny_step = match step_4_break_down() {
        Some(step) => step,
        None => return,
    };

    // Step 5: Celebrate
    step_5_celebrate(&tiny_step);
}

fn step_1_acknowledge() -> bool {
    clear_screen();
    println!();
    println!("{}", "Step 1: Acknowledge".yellow().bold());
    println!();
    println!("You're here, which means you're procrastinating.");
    println!("That's okay. You're not avoiding this task because you don't have time.");
    println!("You're avoiding it for other reasons.");
    println!();

    let continue_prompt = Confirm::new("Ready to explore why?")
        .with_default(true)
        .prompt();

    match continue_prompt {
        Ok(true) => true,
        _ => {
            println!();
            println!("That's okay. Come back when you're ready. 💚");
            println!();
            false
        }
    }
}

fn step_2_examine() -> bool {
    clear_screen();
    println!();
    println!("{}", "Step 2: Examine Your Feelings".yellow().bold());
    println!();
    println!("Why do you have negative feelings about this task?");
    println!("What makes it take so much mental effort?");
    println!();

    let options = vec![
        "The task feels too big and overwhelming",
        "I'm worried about failing or doing it wrong",
        "I don't know where to start",
        "It's boring or uninteresting",
        "I'm afraid of what comes after",
        "My brain just won't let me start (executive dysfunction)",
        "Something else (reflect on your own)",
    ];

    let feeling = Select::new("What resonates with you?", options)
        .with_vim_mode(true)
        .prompt();

    let choice = match feeling {
        Ok(c) => c,
        Err(_) => return false,
    };

    // Clear and show the reflection
    clear_screen();
    println!();
    println!("{}", "Step 2: Examine Your Feelings".yellow().bold());
    println!();
    println!("{}", format!("You said: {}", choice).cyan());
    println!();

    // Provide supportive reflection based on what they chose
    match choice {
        "The task feels too big and overwhelming" => {
            println!("That's a really common feeling. When something feels huge,");
            println!("our brains protect us by making us want to avoid it entirely.");
            println!();
            println!("Remember: You don't have to do the whole thing right now.");
            println!("We'll break it down into something tiny in just a moment.");
        }
        "I'm worried about failing or doing it wrong" => {
            println!("This fear is your brain trying to protect you from disappointment.");
            println!("But here's the thing: not starting guarantees you won't succeed.");
            println!();
            println!("What if you gave yourself permission to do it imperfectly?");
            println!("Done badly is better than not done at all.");
        }
        "I don't know where to start" => {
            println!("Not knowing where to start creates a kind of mental paralysis.");
            println!("Good news: any start is a start. There's no \"wrong\" first step.");
            println!();
            println!("We'll find a tiny, concrete first step together.");
            println!("Sometimes just opening the file is enough.");
        }
        "It's boring or uninteresting" => {
            println!("Your brain craves stimulation, and boring tasks feel like a threat");
            println!("to your ability to stay engaged. That's totally valid.");
            println!();
            println!("Can you pair this task with something enjoyable?");
            println!("Music? A timer game? Treating it like a speed challenge?");
        }
        "I'm afraid of what comes after" => {
            println!("Sometimes we avoid finishing because completion brings change,");
            println!("judgment, or new responsibilities we're not ready for.");
            println!();
            println!("For now, let's just focus on one tiny step.");
            println!("You don't have to think about \"after\" yet.");
        }
        "My brain just won't let me start (executive dysfunction)" => {
            println!("Executive dysfunction is real, and it's not a character flaw.");
            println!("Your brain's 'start button' isn't broken—it just needs different support.");
            println!();
            println!("Sometimes the smallest external structure can help:");
            println!("a timer, a body double, or just naming the tiniest possible step.");
            println!("We'll work on that together.");
        }
        _ => {
            println!("Take a moment to sit with whatever you're feeling.");
            println!("Name it, acknowledge it, let it be there.");
            println!();
            println!("Your feelings are valid, and they're giving you information.");
        }
    }

    println!();
    println!("{}", "💚 These feelings are hard, but you're doing great by naming them.".green());
    println!();

    let continue_prompt = Confirm::new("Ready to improve your mood?")
        .with_default(true)
        .prompt();

    match continue_prompt {
        Ok(true) => true,
        _ => {
            println!();
            println!("Take your time. 💚");
            println!();
            false
        }
    }
}

fn step_3_mood_boost() -> bool {
    clear_screen();
    println!();
    println!("{}", "Step 3: Improve Your Mood".yellow().bold());
    println!();
    println!("Let's take concrete steps to cheer you up a bit.");
    println!();

    let mood_boosters = vec![
        "Do some gentle movement or stretching (whatever feels good)",
        "Put on some music you love",
        "Grab a glass of water or a snack",
        "Move to a different location",
        "Look out the window for 30 seconds",
        "Take 5 deep breaths",
        "Text a friend something nice",
        "Pet your pet (if you have one)",
        "Look at something that makes you happy",
    ];

    let booster = Select::new("Pick something to do right now:", mood_boosters)
        .with_vim_mode(true)
        .prompt();

    match booster {
        Ok(choice) => {
            clear_screen();
            println!();
            println!("{}", "Step 3: Improve Your Mood".yellow().bold());
            println!();
            println!("{}", format!("Great choice: {}", choice).green().bold());
            println!();
            println!("Go ahead and do that now. I'll wait for you...");
            println!();

            let done = Confirm::new("Done?")
                .with_default(true)
                .prompt();

            match done {
                Ok(true) => {
                    clear_screen();
                    println!();
                    // Give them a quick snack as a reward
                    run_quick_snack();
                    println!();
                    true
                }
                _ => {
                    println!();
                    false
                }
            }
        }
        _ => {
            println!();
            false
        }
    }
}

fn step_4_break_down() -> Option<String> {
    loop {
        clear_screen();
        println!();
        println!("{}", "Step 4: Break It Down".yellow().bold());
        println!();
        println!("Let's ignore that big overwhelming task for now.");
        println!("What's the TINIEST thing you could do first?");
        println!();
        println!("Examples:");
        println!("  - Open the file");
        println!("  - Write one sentence");
        println!("  - Read the first paragraph");
        println!("  - Create a new document");
        println!();

        let tiny_step = Text::new("What's your tiny first step?")
            .prompt();

        match tiny_step {
            Ok(step) if !step.trim().is_empty() => {
                clear_screen();
                println!();
                println!("{}", "Step 4: Break It Down".yellow().bold());
                println!();
                println!("{}", format!("Perfect: \"{}\"", step).green().bold());
                println!();

                let commit = Confirm::new("Can you commit to doing JUST this tiny step?")
                    .with_default(true)
                    .prompt();

                match commit {
                    Ok(true) => return Some(step),
                    _ => {
                        clear_screen();
                        println!();
                        println!("{}", "That's okay. Let's try breaking it down even smaller.".yellow());
                        println!();
                        println!("Press Enter to try again, or Ctrl+C to exit.");
                        println!();

                        let _ = Confirm::new("Ready to try again?")
                            .with_default(true)
                            .prompt();

                        // Loop will continue
                    }
                }
            }
            Ok(_) => {
                // Empty input
                clear_screen();
                println!();
                println!("{}", "Step 4: Break It Down".yellow().bold());
                println!();
                println!("I know it can be hard to think of even a tiny step.");
                println!("That feeling of not knowing where to start is really common.");
                println!();
                println!("Here are some ways to make it even smaller:");
                println!("  - Instead of 'write the email', try 'open my email app'");
                println!("  - Instead of 'start the project', try 'create a new file'");
                println!("  - Instead of 'clean the room', try 'pick up one item'");
                println!();
                println!("What's ONE tiny action you could take? Just one.");
                println!();

                let retry = Confirm::new("Want to try again?")
                    .with_default(true)
                    .prompt();

                match retry {
                    Ok(true) => continue, // Loop will retry
                    _ => return None,
                }
            }
            Err(_) => {
                // User cancelled (Ctrl+C or Esc)
                return None;
            }
        }
    }
}

fn step_5_celebrate(tiny_step: &str) {
    clear_screen();
    println!();
    println!("{}", "Step 5: Celebrate!".yellow().bold());
    println!();
    println!("{}", "You just made progress! 🎉".green().bold());
    println!();
    println!("Your tiny step: {}", format!("\"{}\"", tiny_step).cyan());
    println!();

    // ASCII celebration
    println!("{}", r#"
       ★ ━━━━━━━━━━ ★
      ╱|、      🎉
     (˃ᴗ˂ )  HIGH FIVE!
      |、  づ    ✨
     じしˍ)
       ★ ━━━━━━━━━━ ★
    "#.yellow());

    println!("Now, go do that tiny step.");
    println!("Don't look at the whole task. Just do this one small thing.");
    println!();

    let done = Confirm::new("Done with your tiny step?")
        .with_default(true)
        .prompt();

    match done {
        Ok(true) => {
            clear_screen();
            println!();
            println!("{}", "Step 5: Celebrate!".yellow().bold());
            println!();
            println!("{}", "AMAZING! 🚀".green().bold());
            println!();

            let next_step = Text::new("What's the NEXT tiny step you could take?")
                .with_placeholder("(optional - press Enter to skip)")
                .prompt();

            match next_step {
                Ok(step) if !step.trim().is_empty() => {
                    clear_screen();
                    println!();
                    println!("{}", format!("Excellent: \"{}\"", step).cyan());
                    println!();
                    println!("Remember: Baby steps can move mountains. 🏔️");
                    println!();
                }
                _ => {
                    clear_screen();
                    println!();
                    println!("That's okay. You've already made progress! 💪");
                    println!();
                }
            }
        }
        _ => {
            clear_screen();
            println!();
            println!("That's okay. The step is there when you're ready. 💚");
            println!();
        }
    }
}
