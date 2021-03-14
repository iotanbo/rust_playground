// https://github.com/xampprocky/fluent-templates




use std::collections::HashMap;

// use ::phf::{Map, phf_map};


use unic_langid::{LanguageIdentifier, langid};

#[allow(unused)]
use fluent_templates::{Loader, static_loader};

use fluent_templates::fluent_bundle::FluentValue;


const ENG: LanguageIdentifier = langid!("en-US");
const RUS: LanguageIdentifier = langid!("ru");
const UKR: LanguageIdentifier = langid!("uk");


static_loader! {
    static LOC = {
        locales: "./locales",
        fallback_language: "en-US",
        // Removes unicode isolating marks around arguments, you typically
        // should only set to false when testing.
        customise: |bundle| bundle.set_use_isolating(false),
    };
}


fn shared_photos_translate(lang: &LanguageIdentifier, name: &str, gender: &str, count: i32) {
    let repl_map = {
        let mut map: HashMap<String, FluentValue> = HashMap::new();
        map.insert("userName".to_owned(), name.into());
        map.insert("userGender".to_owned(), gender.into());
        map.insert("photoCount".to_owned(), count.into());
        map
    };
    println!(" * {}", LOC.lookup_with_args(&lang, "shared-photos", &repl_map));
 
}


fn shared_photos_translate_demo(lang: &LanguageIdentifier) {
    shared_photos_translate(&lang, "Yuri", "male", 1);
    shared_photos_translate(&lang, "Бодя и Павлик", "other", 3);
    shared_photos_translate(&lang, "Лена", "female", 12);
}


fn main() {

    println!(" == Fluent localization demo begin ==");


    let all_locales = [&ENG, &RUS, &UKR];

    for &loc in &all_locales {

        println!(" * {}", LOC.lookup(&loc, "hello-world"));

        // Translate name
        let localized_person = LOC.lookup(&loc, "Yuri");

        // Create replacement dictionary for template variables
        let repl_map = {
            let mut map: HashMap<String, FluentValue> = HashMap::new();
            map.insert("name".to_owned(), localized_person.into());
            map
        };

        // Process 'greeting' template using replacement dictionary
        println!(" * {}", LOC.lookup_with_args(&loc, "greeting", &repl_map));

        shared_photos_translate_demo(&loc);

    }

    

    println!(" == Fluent localization demo end ==");
}
