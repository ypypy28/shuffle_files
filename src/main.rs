extern crate rand;

use std::fs;
use std::io;

fn random_u32(low: u32, high: u32) -> u32 {
    let mut rng = rand::thread_rng();
    // rand::Rng::gen_range(&mut rng, 100, 1000);
    let n: u32 = rand::Rng::gen_range(&mut rng, low, high);
    n

}

fn read_str() -> String {
    let mut buffer = String::new();

    io::stdin().read_line(&mut buffer).expect("failde to read from stdin");

    buffer.trim().parse().unwrap()

}

fn get_files(ext: &str) -> Vec<fs::DirEntry> {
    let mut files: Vec<fs::DirEntry> = Vec::new();

    for entry in fs::read_dir(".").unwrap() {
        let file: fs::DirEntry = entry.unwrap();
        //println!("{}", file.file_type().unwrap().eq(ext));
        // if file.file_type().unwrap().is_file() {
        if file.path().extension().is_some() {
            if file.path().extension().unwrap().to_string_lossy() == ext {
                // println!("UAAAA");
                // println!("{:?}", file.path());
                // println!("FILE_NAME: {}", file.file_name().to_string_lossy());
                // println!("EXTENTION: {:?}", file.path().extension().unwrap() );
                files.push(file);
            }

        }

    }
    files

}

fn rename(files: Vec<fs::DirEntry>) -> u32 {
    let mut count: u32 = 0;
    for file in files {
        let num = random_u32(0, 999);
        let ren = fs::rename(file.file_name(), format!("{}_{}", num, file.file_name().to_string_lossy()));
        match ren {
            Ok(_) => count +=1,
            Err(..) => {}
        };
        count += 1;
    }
    count
}

fn rename_action(extension: String) {
    let files_to_rename: Vec<fs::DirEntry> = get_files(&extension);
    let files_count = files_to_rename.len();

    if files_count == 0 {
        println!(":: Таких файлов (.{}) в этой папке нет.", extension);

    } else {
        println!(":: Файлов .{} в папке: {} шт. Перемешать? (y|n)", extension, files_count);
        loop {
            let answer = read_str();
            if answer == "y" {
                println!(":: Перемешиваем...");
                let total: u32 = rename(files_to_rename);
                println!(":: Перемешено {} файлов.", total);
                break;
            } else if answer == "n" {
                println!(":: Завершаем работу");
                break
            } else {
                println!(":: Перемешать? (y|n)");
            }
        }
    }

}

fn main() {

    // println!("{}", random_u32(0, 999));
    println!(":: Напиши расширение файлов, которые нужно перемешать\n   (например mp3 или jpg)");
    let extension: String = read_str();

    if extension == "exe"  {
        println!("Давай не будем перемешивать .exe от греха подальше");
    } else {
        rename_action(extension);

    }
    println!("-> Нажмите Enter для выхода");
    read_str();

}
