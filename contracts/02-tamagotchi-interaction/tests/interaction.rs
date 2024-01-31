use gtest::{Program, System};
use tamagotchi_io::*;
use tamagotchi_interaction_io::*;
use gtest::Log;

#[test]
fn interaction_test() {
    let sys = System::new();
    sys.init_logger();
    let program = Program::current(&sys);

    // Inicializa la mascota virtual y asigna un dueño
    program.send(
        1,  // ID del dueño
        TmgAction::Initialize {
            name: "Test Tamagotchi".to_string(),
        },
    );

    // Configura la mascota virtual para evitar valores nulos
    program.send(
        1,  // ID del dueño
        TmgAction::Configure {
            fed: 5000,
            entertained: 5000,
            slept: 5000,
        },
    );

    // Prueba de alimentación (Feed)
    program.send(1, TmgAction::Feed);

    // Verifica que la mascota virtual haya sido alimentada y que el evento sea emitido
    let log_feed = Log::builder().payload(TmgEvent::Fed);
    assert!(program.contains_log(&log_feed));

    // Verifica que el nivel de alimentación sea mayor después de la alimentación
    let state_after_feed: Tamagotchi = program.get_state(2);
    assert!(state_after_feed.fed > 5000);

    // Prueba de entretenimiento (Entertain)
    program.send(1, TmgAction::Entertain);

    // Verifica que la mascota virtual haya sido entretenida y que el evento sea emitido
    let log_entertain = Log::builder().payload(TmgEvent::Entertained);
    assert!(program.contains_log(&log_entertain));

    // Verifica que el nivel de entretenimiento sea mayor después de la acción de entretenimiento
    let state_after_entertain: Tamagotchi = program.get_state(2);
    assert!(state_after_entertain.entertained > 5000);

    // Prueba de sueño (Sleep)
    program.send(1, TmgAction::Sleep);

    // Verifica que la mascota virtual haya dormido y que el evento sea emitido
    let log_sleep = Log::builder().payload(TmgEvent::Slept);
    assert!(program.contains_log(&log_sleep));

    // Verifica que el nivel de sueño sea mayor después de la acción de dormir
    let state_after_sleep: Tamagotchi = program.read_state_state(2);
    assert!(state_after_sleep.slept > 5000);
}
