use printpdf::*;
use std::fs::{self, File};
use std::io::BufWriter;
use std::io;
use std::thread::current;

fn main() {
    // Instructions
    println!(" \n
        Welcome to our PDF writer! \n
        This PDF writer is designed to write letters for any recipient. \n
        Please follow the instructions to write your letter and create your PDF \n.
        ---------------------------------------------- \n");

    let mut array: [String; 14] = Default::default();

    // Function to collect user inputs
    user_input(&mut array);

    // Function to create, save, and open a PDF file
    render_pdf(array);
}

fn user_input(array: &mut [String; 14]) {
    // Name PDF
    println!("Please enter your PDF letter's name: ");
    io::stdin().read_line(&mut array[0]).expect("An error occured with your input:");
    println!();

    // FIRST NAME
    println!("Please enter your first name: ");
    io::stdin().read_line(&mut array[1]).expect("An error occured with your input:");
    println!();

    // Last Name
    println!("Please enter your last name: ");
    io::stdin().read_line(&mut array[2]).expect("An error occured with your input:");
    println!();

    // Street Address
    println!("Please enter your street address: ");
    io::stdin().read_line(&mut array[3]).expect("An error occured with your input:");
    println!();

    // City
    println!("Please enter your city: ");
    io::stdin().read_line(&mut array[4]).expect("An error occured with your input:");
    println!();

    // State
    println!("Please enter your state: ");
    io::stdin().read_line(&mut array[5]).expect("An error occured with your input:");
    println!();

    // Zip
    println!("Please enter your zip code: ");
    io::stdin().read_line(&mut array[6]).expect("An error occured with your input:");
    println!();

    // PHASE #2 - Recipient Info
    println!("\n
        We will now ask for information about who you are sending the letter to. \n
        ------------------------------------------------------------------------ \n");

    // Recipient FIRST NAME
    println!("Please enter the recipient's first name: ");
    io::stdin().read_line(&mut array[7]).expect("An error occured with your input:");
    println!();

    // Recipient LAST NAME
    println!("Please enter the recipient's last name: ");
    io::stdin().read_line(&mut array[8]).expect("An error occured with your input:");
    println!();

    // Recipient Street Address
    println!("Please enter the recipient's street address: ");
    io::stdin().read_line(&mut array[9]).expect("An error occured with your input:");
    println!();

    // Recipient City
    println!("Please enter the recipient's city: ");
    io::stdin().read_line(&mut array[10]).expect("An error occured with your input:");
    println!();

    // Recipient State
    println!("Please enter the recipient's state: ");
    io::stdin().read_line(&mut array[11]).expect("An error occured with your input:");
    println!();

    // Recipient zip code
    println!("Please enter the recipient's zip code: ");
    io::stdin().read_line(&mut array[12]).expect("An error occured with your input:");
    println!();
        
    // LETTER BODY
    println!("Please enter your the text that you wish to include in your letter's body. \n  ");
    io::stdin().read_line(&mut array[13]).expect("An error occured with your input:");
    println!();
}

fn render_pdf(array: [String; 14]) {
    let pdf_title = array[0].trim();
    let file_name = format!("{}.pdf", pdf_title);

    // CREATE DOCUMENT
        let (doc, page1, layer1) = PdfDocument::new(pdf_title, Mm(210.0), Mm(297.0), "Layer 1");
        let current_layer = doc.get_page(page1).get_layer(layer1);

    // FONT
        let font = doc
            .add_builtin_font(BuiltinFont::Helvetica)
            .unwrap();

        let font_weight = doc
            .add_builtin_font(BuiltinFont::HelveticaBold)
            .unwrap();

        let font_italic = doc
            .add_builtin_font(BuiltinFont::HelveticaOblique)
            .unwrap();

    // HEADER LINE
        let points = vec![
            (Point::new(Mm(20.0), Mm(262.0)), false),
            (Point::new(Mm(190.0), Mm(262.0)), false),
        ];
        let line = Line { points, is_closed: false };
        
        current_layer.set_outline_color(Color::Rgb(Rgb::new(0.2, 0.2, 0.2, None)));
        current_layer.set_outline_thickness(1.5);
        current_layer.add_line(line);

    
    // LOGO IMAGE
        
        
         

    // *********************
    // TEXT CONTENT FOR PAGE
    // *********************
    
    // SENDER
    // FIRST + LAST NAME
        current_layer.begin_text_section();
        current_layer.set_font(&font_weight, 10.0);
        current_layer.set_line_height(14.0);
        current_layer.set_text_cursor(Mm(20.0), Mm(280.0));
        current_layer.write_text(format!("{} {}", array[1].trim(), array[2].trim()), &font_weight);
        current_layer.end_text_section();

    // Street Address
        current_layer.begin_text_section();
        current_layer.set_font(&font, 10.0);
        current_layer.set_line_height(14.0);
        current_layer.set_text_cursor(Mm(20.0), Mm(274.0));
        current_layer.write_text(array[3].trim(), &font);
        current_layer.end_text_section();

    // City, State, Zip
        current_layer.begin_text_section();
        current_layer.set_font(&font, 10.0);
        current_layer.set_line_height(14.0);
        current_layer.set_text_cursor(Mm(20.0), Mm(268.0));
        current_layer.write_text(format!("{}, {} {}", array[4].trim(), array[5].trim(), array[6].trim()), &font);
        current_layer.end_text_section();

    // RECIPIENT 
    // First + Last Name
        current_layer.begin_text_section();
        current_layer.set_font(&font_weight, 11.0);
        current_layer.set_line_height(14.0);
        current_layer.set_text_cursor(Mm(20.0), Mm(250.0)); // Moved below the title item
        current_layer.write_text(format!("{} {}", array[7].trim(), array[8].trim()), &font_weight);
        current_layer.end_text_section();

    // Street Address
        current_layer.begin_text_section();
        current_layer.set_font(&font, 11.0);
        current_layer.set_line_height(14.0);
        current_layer.set_text_cursor(Mm(20.0), Mm(244.0));
        current_layer.write_text(array[9].trim(), &font);
        current_layer.end_text_section();

    // City, State, Zip
        current_layer.begin_text_section();
        current_layer.set_font(&font, 11.0);
        current_layer.set_line_height(14.0);
        current_layer.set_text_cursor(Mm(20.0), Mm(238.0));
        current_layer.write_text(format!("{}, {} {}", array[10].trim(), array[11].trim(), array[12].trim()), &font);
        current_layer.end_text_section();

    // --- LETTER BODY ---
    // Dear '___'
        current_layer.begin_text_section();
        current_layer.set_font(&font, 12.0);
        current_layer.set_line_height(16.0);
        current_layer.set_text_cursor(Mm(20.0), Mm(220.0));
        current_layer.write_text(format!("Dear {} {},", array[7].trim(), array[8].trim()), &font);
        current_layer.end_text_section();

    // BODY
        current_layer.begin_text_section();
        current_layer.set_font(&font, 12.0);
        let line_height_pt = 16.0; 
        current_layer.set_line_height(line_height_pt);

    // Track our starting vertical Y coordinate
    let mut current_y = 208.0; 
    
    // Break the long string into wrapped chunks (65 characters per line is safe for 170mm width)
    let wrapped_lines = wrap_text(array[13].trim(), 85);

    for line_content in wrapped_lines {
        // Stop body from growing below a certain height.
        if current_y < 50.0 { 
            break; 
        }

        current_layer.begin_text_section();
        current_layer.set_font(&font, 12.0);
        current_layer.set_line_height(line_height_pt);
        current_layer.set_text_cursor(Mm(20.0), Mm(current_y));
        current_layer.write_text(line_content, &font);
        current_layer.end_text_section();
        
        current_y = current_y - 6.0; 
    }

    // SINCERELY
    current_layer.begin_text_section();
    current_layer.set_font(&font, 12.0);
    current_layer.set_line_height(line_height_pt);
    current_layer.set_text_cursor(Mm(20.0), Mm(current_y - 6.0));
    current_layer.write_text("Sincerely, ", &font);
    current_layer.end_text_section();

    // SENDER NAME SIGN-OFF
    current_layer.begin_text_section();
    current_layer.set_font(&font, 12.0);
    current_layer.set_line_height(line_height_pt);
    current_layer.set_text_cursor(Mm(20.0), Mm(current_y - 12.0));
    current_layer.write_text(format!("{} {}", array[1], array[2]), &font);
    current_layer.end_text_section();

    
    // PDF TITLE IN FOOTER
    current_layer.begin_text_section();
    current_layer.set_font(&font_italic, 9.0);
    current_layer.set_text_cursor(Mm(160.0), Mm(20.0));
    current_layer.write_text(pdf_title, &font_italic);
    current_layer.end_text_section();


    // SAVE DOCUMENT TO FILE
    let folder_path = format!("pdf_reports/{}", file_name);
    let mut file = BufWriter::new(File::create(&folder_path).unwrap());
    doc.save(&mut file).unwrap();

    // AUTO-OPEN
    std::process::Command::new("cmd")
        .args(["/C", "start", "", &folder_path])
        .spawn()
        .unwrap();
}

fn wrap_text(text: &str, max_chars_per_line: usize) -> Vec<String> {
    let mut lines = Vec::new();
    let mut current_line = String::new();

    for word in text.split_whitespace() {
        if current_line.len() + word.len() + 1 > max_chars_per_line {
            if !current_line.is_empty() {
                lines.push(current_line.clone());
                current_line.clear();
            }
        }
        
        if !current_line.is_empty() {
            current_line.push(' ');
        }
        current_line.push_str(word);
    }

    if !current_line.is_empty() {
        lines.push(current_line);
    }

    lines
}



