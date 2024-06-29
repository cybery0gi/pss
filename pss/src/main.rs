use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]

//त्रिषष्टिश्चतुःषटिर्वा वर्णाः शम्भुमतेमताः ।
//प्राकृते संस्कृते चापि स्वयं प्रोक्ताः स्वयम्भुवा ॥ ३॥
// construct an enum of 64 Varnas.
pub enum Varna {
//स्वरा विंशतिरेकश्च स्पर्शानां पञ्चविंशतिः ।
//यादयश्च स्मृता ह्यष्टौ चत्वारश्च यमाः स्मृताः ॥ ४॥
// define 21 Svaras, 25 Sparshas, 8 "ya"-adayahs, 4 Yamas, and an anusvara,visarga and pluta.

    Svara1, Svara2, Svara3, Svara4, Svara5,
    Svara6, Svara7, Svara8, Svara9, Svara10,
    Svara11, Svara12, Svara13, Svara14, Svara15,
    Svara16, Svara17, Svara18, Svara19, Svara20, Svara21,

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
    // Example usage: Serialize and deserialize a Varna variant
    let varna = Varna::Svara1;
    let serialized = serde_json::to_string(&varna).unwrap();
    println!("Serialized: {}", serialized);

    let deserialized: Varna = serde_json::from_str(&serialized).unwrap();
    println!("Deserialized: {:?}", deserialized);
}
