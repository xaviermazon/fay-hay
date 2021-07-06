fn transform(word: &mut String) {

    let caracteres = word.chars().nth(0).unwrap();
    
    if is_vowel(caracteres) {
	word.push_str("-hay");
    } else {
	let consonant = caracteres;
	word.remove(0);
	let suffix = format!("-{}ay",consonant);
	word.push_str(&suffix);
    }
}

fn is_vowel(letra: char) -> bool {
    letra == 'a' || letra == 'e' || letra == 'i' || letra == 'o' || letra == 'u'
}

fn main() {
    let mut word = String::from("ooola");
    println!("Palabra a transformar: {}", word);
    transform(&mut word);
    println!("Palabra transformada: {}", word);
}
