use winit::{
    event::{ElementState, Event, MouseButton, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use rfd::MessageDialog;
use fs_extra::dir::{copy, CopyOptions};
use std::io;
use winit::dpi::LogicalPosition;


// Funzione per mostrare una finestra di conferma usando la libreria rfd
fn show_confirmation_dialog() -> bool {
    MessageDialog::new()
        .set_title("Conferma Backup")
        .set_description("Vuoi davvero iniziare il backup?")
        .set_buttons(rfd::MessageButtons::YesNo)
        .show()
}

// Funzione per eseguire il backup usando fs_extra
fn perform_backup(src: &str, dst: &str) -> io::Result<()> {
    let options = CopyOptions::new();
    copy(src, dst, &options).map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
    Ok(())
}

// Struttura per memorizzare i clic del mouse
#[derive(Default)]
struct MouseSymbolDetector {
    clicks: Vec<(f64, f64)>, // Memorizza le coordinate (x, y) dei clic del mouse
}

impl MouseSymbolDetector {
    // Funzione per controllare se i clic del mouse formano un quadrato
    fn is_square(&self) -> bool {
        if self.clicks.len() == 4 {
            // Controllo se i punti formano un quadrato
            let p1 = self.clicks[0];
            let p2 = self.clicks[1];
            let p3 = self.clicks[2];
            let p4 = self.clicks[3];

            // Calcolo delle distanze tra i punti
            fn d2(a: (f64, f64), b: (f64, f64)) -> f64{
                let x1 = a.0;
                let y1 = a.1;
                let x2 = b.0;
                let y2 = b.1;
                (x1 - x2).powi(2) + (y1 - y2).powi(2)
            }

            let d2_12 = d2(p1, p2);
            let d2_13 = d2(p1, p3);
            let d2_14 = d2(p1, p4);

            if d2_12 == d2_13 && d2_14 == 2.0 * d2_12 && d2(p2, p4) == d2_12 && d2(p3, p4) == d2_12 {
                return true;
            }
        }
        false
    }
}

// Funzione principale dell'applicazione
fn main() {
    // Crea un nuovo event loop
    let event_loop = EventLoop::new();
    // Crea una nuova finestra
    let window = WindowBuilder::new()
        .with_title("Backup Tool")
        .build(&event_loop)
        .unwrap();

    // Variabile per rilevare il disegno del simbolo con il mouse
    let mut symbol_detector = MouseSymbolDetector::default();

    // Avvia l'event loop
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        // Gestione degli eventi
        match event {
            // Eventi della finestra
            Event::WindowEvent { event, .. } => match event {
                // Richiesta di chiusura della finestra
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,

                // Eventi di input del mouse
                WindowEvent::MouseInput { button, state, .. } => {
                    if button == MouseButton::Left {
                        // Se il pulsante sinistro del mouse è premuto
                        match state {
                            ElementState::Pressed => {
                                /*
                                // Aggiungere la posizione del clic del mouse al rilevatore del simbolo
                                let cursor_position = LogicalPosition::new(0.0, 0.0);
                                window.set_cursor_position(cursor_position).unwrap_or((0.0, 0.0));
                                symbol_detector.clicks.push(cursor_position);


                                // Verifica se i clic del mouse formano un quadrato
                                if symbol_detector.is_square() {
                                    // Se i clic del mouse formano un quadrato, mostra la finestra di dialogo di conferma
                                    if show_confirmation_dialog() {
                                        // Esegui il backup
                                        let src_path = "C:\\Users\\Studio\\Desktop\\prova";
                                        let dst_path = "C:\\Users\\Studio\\Desktop\\dest";

                                        if let Err(e) = perform_backup(src_path, dst_path) {
                                            eprintln!("Errore durante il backup: {:?}", e);
                                        }
                                    }

                                    // Svuota i clic del mouse dopo aver rilevato il simbolo
                                    symbol_detector.clicks.clear();
                                }
                                */

                                if show_confirmation_dialog() {
                                    let src_path = "C:\\Users\\Studio\\Desktop\\prova";
                                    let dst_path = "C:\\Users\\Studio\\Desktop\\dest";

                                    if let Err(e) = perform_backup(src_path, dst_path) {
                                        eprintln!("Errore durante il backup: {:?}", e);
                                    }
                                }

                            }
                            _ => (),
                        }
                    }
                }
                _ => (),
            },
            _ => (),
        }
    });
}
