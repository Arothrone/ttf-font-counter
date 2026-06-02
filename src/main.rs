
use ttf_parser::{Face, GlyphId};

fn main() {
    let font_data = std::fs::read("C:/Windows/Fonts/times.ttf")//getting Times New Roman
        .expect("Cannot read file.");
    
    let face = Face::parse(&font_data, 0)
        .expect("Font Parsing Error");

    let em = face.units_per_em() as f32; // get default Units per EM

    let mut widths = Vec::new();
    
    for id in 0..face.number_of_glyphs() {
        let glyph_id = GlyphId(id);


        if let Some(bbox) = face.glyph_bounding_box(glyph_id) { // https://docs.rs/ttf-parser/latest/ttf_parser/struct.Face.html explains everything on its own. Constant width per charater.
            let width_units = bbox.x_max - bbox.x_min;
            
            let const_width = width_units as f32 / em;

            widths.push(const_width);
            //iterating all widths of all Times New Roman glyphs.
        }
    }



    let ascender_ratio = face.ascender() as f32;
    let descender_ratio = face.descender() as f32;
    let line_gap_ratio = face.line_gap() as f32; // https://learn.microsoft.com/en-us/typography/opentype/spec/recom#tad sTypoAscender, sTypoDescender and sTypoLineGap and formula

    let mut total_line_height = (ascender_ratio - descender_ratio + line_gap_ratio) / em; // getting line height. Formula here: https://learn.microsoft.com/en-us/typography/opentype/spec/recom#tad
    println!("Constant line height in word: {}", total_line_height);


    println!("--------------------------------------------------");

    
    if !widths.is_empty() {

        let mut finavg = 0 as f32;

        let sum: f32 = widths.iter().sum();
        let average_width = sum / widths.len() as f32; // finding average width per glyph.

        println!("All symbols with em box: {}", widths.len());
        println!("Average constant em box width in Times New Roman: {:.4}", average_width);

        // let page_count = 1 as f32;
        // let font_size = 12.0;
        // let sq = 8.5 * 11.0; // default format  “letter” https://www.windwardstudios.com/blog/page-size-and-orientation-in-word   https://en.wikipedia.org/wiki/Letter_(paper_size)

        // let symb_sq = average_width * (1.0 / 72.0) * font_size * total_line_height * (1.0 / 72.0) * font_size;



        // let total_count: f32 = (sq * page_count) / symb_sq;

        
        let mut sq = (8.5 as f32) * (11.0 as f32) as f32; // default format  “letter” https://www.windwardstudios.com/blog/page-size-and-orientation-in-word   https://en.wikipedia.org/wiki/Letter_(paper_size)
        let page_count = 5 as f32;

        let total_count = 99.34838822*5 as f32;// average sentence length in 2000 and across genres plus punctuation plus spaces = 97.66804314

        let mut font_size = (72 as f32)*((sq*page_count)/(total_count*average_width*total_line_height) as f32).sqrt() as f32;

        finavg += font_size;

        println!("final font no margin no default line spacing {:.5}", font_size);
        

        // sq = (8.5 - 1 as f32) * (11.0 - 1 as f32) as f32; // https://support.microsoft.com/en-us/office/change-margins-da21a474-99d8-4e54-b12d-a8a14ea7ce02 default margins

        // font_size = (72 as f32)*((sq*page_count)/(total_count*average_width*total_line_height) as f32).sqrt() as f32;

        // finavg += font_size;

        // println!("final font 1 inch margin no default line spacing {:.5}", font_size);

        total_line_height *= 1.15; // https://support.microsoft.com/en-us/office/change-the-default-line-spacing-in-word-411437a0-0646-490d-b426-a9249a78b315 default line spacing 1.15

        // sq = (8.5 as f32) * (11.0 as f32) as f32;
        // font_size = (72 as f32)*((sq*page_count)/(total_count*average_width*total_line_height) as f32).sqrt() as f32;

        // finavg += font_size;

        // println!("final font no margin with default line spacing {:.5}", font_size);
        

        sq = (8.5 - 1 as f32) * (11.0 - 1 as f32) as f32;

        font_size = (72 as f32)*((sq*page_count)/(total_count*average_width*total_line_height) as f32).sqrt() as f32;

        finavg += font_size;

        println!("final font 1 inch margin with Word default line spacing {:.5}", font_size);


        finavg /= 2 as f32;

        println!("avg for everything {:.5}", finavg);
        
    } else {
        println!("No characters with em square found.");
    }
    
}


//https://learn.microsoft.com/en-us/typography/opentype/spec/ttch01 -- yes, I read this up to the part Converting FUnits to pixels and understood it completely. As I didn't know rust (But I did know C++), this code can be called ai-assisted. I controlled every line of code and understand it all.