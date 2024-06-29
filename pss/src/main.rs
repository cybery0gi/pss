use serde::{Serialize, Deserialize};
mod svara;

use svara::{Svara, SvaraEnum};

#[derive(Serialize, Deserialize, Debug)]
pub enum Varna {
    Svara(Svara),

    // Sparshas (Consonants)
    Sparsha1, Sparsha2, Sparsha3, Sparsha4, Sparsha5,
    Sparsha6, Sparsha7, Sparsha8, Sparsha9, Sparsha10,
    Sparsha11, Sparsha12, Sparsha13, Sparsha14, Sparsha15,
    Sparsha16, Sparsha17, Sparsha18, Sparsha19, Sparsha20,
    Sparsha21, Sparsha22, Sparsha23, Sparsha24, Sparsha25,

    // Yadaya (Semi-vowels)
    Yadaya1, Yadaya2, Yadaya3, Yadaya4, Yadaya5,
    Yadaya6, Yadaya7, Yadaya8,

    // Yama (Sibilants)
    Yama1, Yama2, Yama3, Yama4,

    // Anusvara
    Anusvara,

    // Visarga
    Visarga,

    // Pluta
    Pluta,
}

fn main() {
    // Example usage: Serialize and deserialize a Varna variant with Svara
    let svara = Svara::new(SvaraEnum::Svara1);
    let varna = Varna::Svara(svara);
    let serialized = serde_json::to_string(&varna).unwrap();
    println!("Serialized: {}", serialized);

    let deserialized: Varna = serde_json::from_str(&serialized).unwrap();
    println!("Deserialized: {:?}", deserialized);
}
