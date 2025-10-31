// use std::time::Duration;

// use crate::engines::db::{start_db_server, db_query};
// use crate::engines::web::{start_web_server};

// use std::{any::Any, fmt::Error, io::{Write, stdin, stdout}, path::Path};

// use std::error::Error;

// use genpdf::{Context, Element, Mm, Position, RenderResult, style::{Color, Style}};
// use rusqlite::Connection;

// use crate::engines::db::{lexer::Lexer, sql, tokens::TokenKinds};

use std::path::Path;


mod engines;

// struct LineElement {
//     start: (Mm, Mm),
//     end: (Mm, Mm),
//     color: Color,
//     width: Mm,
// }

// impl Element for LineElement {
//     fn render(
//             &mut self,
//             context: &Context,
//             area: genpdf::render::Area<'_>,
//             style: Style,
//         ) -> Result<genpdf::RenderResult, genpdf::error::Error> {
//         let mut points = Vec::new();    
//         points.push(Position { x: self.start.0, y: self.start.1 });
//         points.push(Position { x: self.end.0, y: self.end.1 });
//         let _ = area.draw_line(points,style.with_color(self.color)); //(self.start, self.end, self.width, self.color);
//         Ok(RenderResult{size: area.size(),has_more:false})
//     }
// }
#[derive(Debug)]
struct Person {
    id: i32,
    name: String
}


fn main() {

    let con = nanosql::Connection::open(Path::new("./db.sqlite")).expect("failed to open connection !");
    // con.execute("create table test (id int, name varchar(100));",()).expect("failed to execute sql");
    // con.execute("insert into test (id,name) select 1,'go';",()).expect("failed to execute sql");
    // con.execute("insert into test (id,name) select 2,'rust';",()).expect("failed to execute sql");

    let mut stmt: nanosql::Statement<'_> = con.prepare("select * from test").expect("failed to run sql");

    let rows = stmt.query_map([],|r| Ok(Person{id:r.get_unwrap(0),name:r.get_unwrap(1)})).unwrap();
    {
        
        for person in rows {
            if let Ok(p) = person {
                println!("{:?}",p);
            }
        }
    }


    // Ok(())

}


//     // Load a font from the file system
//     let font_family = genpdf::fonts::from_files("./assets/fonts", "times", None)
//         .expect("Failed to load font family");
//     // Create a document and set the default font family
//     let mut doc = genpdf::Document::new(font_family);
//     // Change the default settings
//     doc.set_title("Demo document");
//     // Customize the pages
//     let mut decorator = genpdf::SimplePageDecorator::new();
//     decorator.set_margins(10);
//     doc.set_page_decorator(decorator);
//     // Add one or more elements
//     doc.push(genpdf::elements::Paragraph::new("This is a demo document."));
//     doc.push(LineElement {
//         start: (Mm::from(0),Mm::from(0)),
//         end: (Mm::from(410),Mm::from(410)),
//         color: Color::Rgb(0, 0, 0),
//         width: Mm::from(30)
//     });
//     // Render the document and write it to a file
//     doc.render_to_file("output.pdf").expect("Failed to write PDF file");

    

// }

// #[allow(unused)]
// fn db() -> std::io::Result<()> {
//     let mut db = String::new();

//     loop {
//         print!("{}> ", db);
//         stdout().flush()?;
//         let mut buf : String = String::new();
//         stdin().read_line(&mut buf)?;
//         let lex = Lexer::new(&buf);
//         // if let Some(err) = grammer::get_error(db.as_str(), &lex) {
//         //     println!("[FAILED] {err}");
//         //     continue;
//         // }
//         match lex.tokens[0].kind {
//             &TokenKinds::USE => {
//                 let folder = format!("{}/{}", sql::HOME_DIR, lex.tokens[1].val);
//                 let path = Path::new(folder.as_str());
//                 if !path.exists() {
//                     eprintln!("SQL ERROR: incorrect db [{}]", lex.tokens[1].val);
//                     continue;
//                 }
//                 db = format!("{}", lex.tokens[1].val);
//                 continue;
//             },
//             &TokenKinds::QUIT|&TokenKinds::EXIT => {
//                 break;
//             },
//             _ => {}
//         }        
//         for token in &lex.tokens {
//             println!("{}", token.to_string());
//         }
//         println!("{}", sql::execute_sql(&db,&lex).unwrap_or("failed !!! for some unknown reason".into()));
//     }
    
//     println!("good bye !");

//     Ok(())
    
// }




