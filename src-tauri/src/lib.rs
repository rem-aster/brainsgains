use tauri_plugin_sql::{Migration, MigrationKind};

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn vec_migrations() -> Vec<tauri_plugin_sql::Migration> {
    return vec![Migration {
        version: 1,
        description: "create_initial_tables",
        sql: "
CREATE TABLE IF NOT EXISTS cards (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    front TEXT NOT NULL,
    back TEXT NOT NULL,
    pack_id INTEGER NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS packs (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    description TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TRIGGER IF NOT EXISTS update_cards_timestamp
AFTER UPDATE ON packs
FOR EACH ROW
BEGIN
    UPDATE cards SET updated_at = CURRENT_TIMESTAMP WHERE id = OLD.id;
END;

CREATE TRIGGER IF NOT EXISTS update_pack_timestamp
AFTER UPDATE ON packs
FOR EACH ROW
BEGIN
    UPDATE packs SET updated_at = CURRENT_TIMESTAMP WHERE id = OLD.id;
END;

CREATE INDEX IF NOT EXISTS idx_cards_id ON cards(pack_id);
CREATE INDEX IF NOT EXISTS idx_cards_front ON cards(front COLLATE NOCASE);
CREATE INDEX IF NOT EXISTS idx_cards_back ON cards(back COLLATE NOCASE);
CREATE INDEX IF NOT EXISTS idx_cards_pack_id ON cards(pack_id);
CREATE UNIQUE INDEX IF NOT EXISTS idx_packs_name_unique ON packs(name);
",
        kind: MigrationKind::Up,
    },
    Migration {
        version: 2,
        description: "demo_data",
        sql: "
INSERT INTO packs (name, description) VALUES 
    ('French Basics', 'Essential French vocabulary for beginners'),
    ('Math Formulas', 'Common mathematical formulas to remember'),
    ('Science Terms', 'Important scientific terminology');

-- Insert demo cards for French Basics
INSERT INTO cards (front, back, pack_id) VALUES
    ('Bonjour', 'Hello', 1),
    ('Merci', 'Thank you', 1),
    ('Au revoir', 'Goodbye', 1),
    ('Comment ça va?', 'How are you?', 1),
    ('Je m''appelle...', 'My name is...', 1);

-- Insert demo cards for Math Formulas
INSERT INTO cards (front, back, pack_id) VALUES
    ('Area of circle', 'πr²', 2),
    ('Pythagorean theorem', 'a² + b² = c²', 2),
    ('Quadratic formula', 'x = [-b ± √(b²-4ac)]/2a', 2),
    ('Slope formula', '(y₂-y₁)/(x₂-x₁)', 2);

-- Insert demo cards for Science Terms
INSERT INTO cards (front, back, pack_id) VALUES
    ('Photosynthesis', 'Process by which plants convert light to energy', 3),
    ('Mitosis', 'Cell division process', 3),
    ('Newton''s First Law', 'An object in motion stays in motion', 3),
    ('DNA', 'Deoxyribonucleic acid', 3),
    ('Atom', 'Basic unit of matter', 3);
        ",
        kind: MigrationKind::Up,
    }
    ];
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let mut builder = tauri::Builder::default()
        .plugin(
            tauri_plugin_sql::Builder::default()
                .add_migrations("sqlite:cards.db", vec_migrations())
                .build(),
        )
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_opener::init());
    
    #[cfg(not(any(target_os = "android", target_os = "ios")))]
    {
        builder = builder.plugin(tauri_plugin_window_state::Builder::default().build())
    }

    builder.invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
