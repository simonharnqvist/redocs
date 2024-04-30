use std::io::Stdout;
use std::path::PathBuf;
use pyo3::prelude::*;
use pyo3::Bound;
use build_html::*;
use std::fs;


/// Figure to insert into HTML
#[derive(Debug)]
struct Figure {
    path: String,
    caption: String,
    caption_size:String,
    caption_font:String,
    caption_color:String,
    caption_background:String,
    style: String 
}

impl Figure {
    pub fn new(path: PathBuf, caption: Option<String>, 
                caption_size: Option<u8>, caption_font: Option<String>,
                caption_color: Option<String>, caption_background: Option<String>,
                css: Option<String>) -> Self {

        let path = path.clone().into_os_string().into_string().unwrap();
        let caption = if caption.is_some() {caption.unwrap()} else {"".to_string()};
        let caption_size: String = if caption_size.is_some() {caption_size.unwrap().to_string()} else {"".to_string()};
        let caption_font: String = if caption_font.is_some() {caption_font.unwrap().to_string()} else {"".to_string()};
        let caption_color: String = if caption_color.is_some() {caption_color.unwrap().to_string()} else {"".to_string()};
        let caption_background: String = if caption_background.is_some() {caption_background.unwrap().to_string()} else {"".to_string()};
        let css = if css.is_some() {css.unwrap()} else {"".to_string()};

        Figure { path,
                caption,
                caption_size,
                caption_font,
                caption_color,
                caption_background,
                style: css }
    }
}

impl Html for Figure {
    fn to_html_string(&self) -> String {
        format!("<figure><img src={0} alt={1} style={2}><figcaption>{1}</figcaption></figure>", 
                    self.path, self.caption, self.style)
    }
}

/// Header object
#[derive(Debug)]
struct Header {
    text: String,
    level: u8,
    text_align:String,
    font_family: String,
    color: String,
    background: String,
    style: String
}

impl Header {
    pub fn new(text: String, level:u8, text_align: Option<String>,
                font: Option<String>, col: Option<String>, background: Option<String>, css: Option<String>) -> Header {

                let text_align: String = if text_align.is_some() {text_align.unwrap()} else {"".to_string()};
                let font_family: String = if font.is_some() {font.unwrap()} else {"".to_string()};
                let color: String = if col.is_some() {col.unwrap()} else {"".to_string()};
                let background: String = if background.is_some() {background.unwrap()} else {"".to_string()};
                let css: String = if css.is_some() {css.unwrap()} else {
                    format!("'text-align: {0}; font-family: {1}; color: {2}; background: {3}'",
                    text_align, font_family, color, background)
                };


                    Header {
                        text,
                        level,
                        text_align,
                        font_family,
                        color,
                        background,
                        style:css
                    }
                }
}

impl Html for Header {
    fn to_html_string(&self) -> String {
        format!(r"<h{0} style={1}>{2}</h1>", 
            self.level, self.style, self.text)
    }
}

#[derive(Debug)]
struct Paragraph {
    text: String,
    size: String,
    text_align:String,
    font_family: String,
    color: String,
    background: String,
    style: String
}

impl Paragraph {
    pub fn new(text: String, size: Option<u8>, text_align: Option<String>,
                font_family: Option<String>, color: Option<String>, background: Option<String>, css: Option<String>) -> Paragraph {

                let text_align: String = if text_align.is_some() {text_align.unwrap()} else {"".to_string()};
                let size:String = if size.is_some()  {size.unwrap().to_string()} else {"".to_string()};
                let font_family: String = if font_family.is_some() {font_family.unwrap()} else {"".to_string()};
                let color: String = if color.is_some() {color.unwrap()} else {"".to_string()};
                let background: String = if background.is_some() {background.unwrap()} else {"".to_string()};
                let css: String = if css.is_some() {css.unwrap()} else {
                    format!("'text-align: {0}; font-family: {1}; color: {2};     background: {3}'",
                                text_align, font_family, color, background)
                };


                    Paragraph {
                        text,
                        size,
                        text_align,
                        font_family,
                        color,
                        background,
                        style:css
                    }
                }
}

impl Html for Paragraph {
    fn to_html_string(&self) -> String {
        format!("<p style={0}>{1}</p>", self.style, self.text)
    }
}

/// HTML report object.
#[pyclass]
pub struct HTMLReport {
    pub title: String,
    pub page: HtmlPage,
    pub style: String,
}


#[pymethods]
impl HTMLReport {
    #[new]
    fn py_new(title:String, style:Option<String>) -> PyResult<Self> {
        let page = build_html::HtmlPage::new();
        let style = if style.is_some() { style.unwrap() } else { "".to_string() };
        Ok(HTMLReport { title, page, style })
    }

    // STYLE OPTIONS

    /// Add style options
    fn add_style(&mut self, css: String) {
        self.style.push_str(&css)

    }

    /// Set default font for document
    fn set_font(&mut self, fontname: String) {
        let font_css: String = format!("p{{font-family:{0};}}", fontname);
        self.add_style(font_css)
    }

    fn set_background(&mut self, background: String) {
        let background_css: String = format!("p{{background:{0};}}", background);
        self.add_style(background_css)
    }
    
    /// Insert raw CSS
    fn insert_css(&mut self, css: String) {
        self.add_style(css)
    }


    // CONTENT INSERTIONS

    /// Add a header to document
    fn add_header(&mut self, text: String, level: u8, text_align: Option<String>,
                    font_family: Option<String>, color:Option<String>, background: Option<String>,
                    css: Option<String>) {
        let header_html: String = Header::new(text, level, text_align, font_family, color, background, css).to_html_string();
        self.page.add_html(header_html)
    }

    /// Add a paragraph of text to document
    fn add_paragraph(&mut self, text: String, size:Option<u8>, text_align: Option<String>,
        font_family: Option<String>, color:Option<String>, background: Option<String>,
        css: Option<String>) {
        
        let paragraph_html: String = Paragraph::new(text, size, text_align, font_family, color, background, css).to_html_string();
        self.page.add_html(paragraph_html)
    }

    fn add_table(&mut self, html_table: String, css: Option<String>) {
        self.page.add_raw(html_table);

        let css: String = if css.is_some() {css.unwrap()} else {
            format!("table {{font-family: Arial; border-collapse: collapse; width: 50%}}")};

        self.add_style(css)

    }

    /// Add image from path
    fn add_figure(&mut self, path: PathBuf, 
                    caption: Option<String>,
                    caption_size: Option<u8>, 
                    caption_font: Option<String>,
                    caption_color: Option<String>, 
                    caption_background: Option<String>,
                    css: Option<String>) {

        let fig_html: String = Figure::new(
            path,
            caption,
            caption_size,
            caption_font,
            caption_color,
            caption_background,
            css).to_html_string();


        self.page.add_html(fig_html);

                    }



    // OUTPUT/RENDERING FUNCTIONS

    /// Print HTML string
    fn to_html(&mut self) -> String {
        self.page.add_style(&self.style);
        self.page.to_html_string()
    }

    /// Render document as HTML
    fn render(&mut self, path: PathBuf) -> () {
        let html_string: String = self.to_html();
        fs::write(path, html_string).expect("Couldn't write to file");
    }
}

#[pymodule]
fn redocs(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<HTMLReport>()?;
    Ok(())
}
