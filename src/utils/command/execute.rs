use std::process::Command;
use std::thread;
use std::time::Duration;

pub fn command(description: &str, command: &str) {
    match Command::new("cmd")
        .args(["/C", command])
        .spawn()
    {
        Ok(mut child) => {
            println!("{} lancé avec succès.", description);
            if description.to_lowercase().contains("stop") {
                let _ = child.wait(); // S'assurer que le processus d'arrêt est complet
                thread::sleep(Duration::from_secs(2)); // Délai pour garantir que nginx est bien terminé
            }
            match child.try_wait() {
                Ok(Some(status)) => println!("Process exited with: {}", status),
                Ok(None) => println!("{} is running in the background.", description),
                Err(e) => eprintln!("Error waiting for {}: {}", description, e),
            }
        },
        Err(e) => eprintln!("Error executing {}: {}", description, e),
    }
}