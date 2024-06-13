#[derive(Debug)]
enum Varna {
    Vowel(Vowel),
    Consonant(Consonant),
    Semivowel(Semivowel),
    Sibilant(Sibilant),
}

// Define the different types of Vowels
#[derive(Debug)]
enum Vowel {
    ShortA,
    LongA,
    ShortI,
    LongI,
    ShortU,
    LongU,
    ShortR,
    LongR,
    ShortE,
    LongE,
    ShortAi,
    LongAi,
    ShortO,
    LongO,
    ShortAu,
    LongAu,
    // Additional vowels
    ShortI_1,
    LongI_1,
    ShortU_1,
    LongU_1,
    ShortE_1,
    LongE_1,
    ShortO_1,
    LongO_1,
}

// Define the different types of Consonants
#[derive(Debug)]
enum Consonant {
    // Examples of consonants
    K,
    Kh,
    G,
    Gh,
    Ng,
    C,
    Ch,
    J,
    Jh,
    Ny,
    T,
    Th,
    D,
    Dh,
    N,
    Tt,
    Tth,
    Dd,
    Ddh,
    Nn,
    P,
    Ph,
    B,
    Bh,
    Mh,
    Y,
    R,
    L,
    V,
    Sh,
    S,
    H,
    Lh,
    Rs,
    Z,
    Tt_1,
    Tth_1,
    Dd_1,
    Ddh_1,
    Nn_1,
}

// Define the different types of Semivowels
#[derive(Debug)]
enum Semivowel {
    Y,
    R,
    L,
    V,
    W,
    H,
    Lh,
    Rs,
}

// Define the different types of Sibilants
#[derive(Debug)]
enum Sibilant {
    S,
    Sh,
    Shh,
    Z,
}

fn main() {
    let my_vowel = Varna::Vowel(Vowel::LongA);
    let my_consonant = Varna::Consonant(Consonant::Kh);
    let my_semivowel = Varna::Semivowel(Semivowel::R);
    let my_sibilant = Varna::Sibilant(Sibilant::Sh);

    println!("Varna: {:?}", my_vowel);
    println!("Varna: {:?}", my_consonant);
    println!("Varna: {:?}", my_semivowel);
    println!("Varna: {:?}", my_sibilant);
}
