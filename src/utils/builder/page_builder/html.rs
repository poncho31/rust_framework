use std::collections::HashMap;

// TODO : 1) FAIRE un schema des relations entres éléments html afin de les implémenter dans tera 
//        2) Comment je veux les afficher afin de créer un objet représentant les métadonnées qui sera injecté dans le moteur de template Tera
//        3) l'affichage de ces éléments se fera via me moteur de template tera
#[derive(Debug)]
pub enum Tag {
    A,
    Abbr,
    Address,
    Area,
    Article,
    Aside,
    Audio,
    B,
    Base,
    Bdi,
    Bdo,
    Blockquote,
    Body,
    Br,
    Button,
    Canvas,
    Caption,
    Cite,
    Code,
    Col,
    Colgroup,
    Data,
    Datalist,
    Dd,
    Del,
    Details,
    Dfn,
    Dialog,
    Div(Div),
    Dl,
    Dt,
    Em,
    Embed,
    Fieldset,
    Figcaption,
    Figure,
    Footer,
    Form,
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
    Head,
    Header,
    Hr,
    Html(Html),
    I,
    Iframe,
    Img,
    Input,
    Ins,
    Kbd,
    Label,
    Legend,
    Li,
    Link,
    Main,
    Map,
    Mark,
    Meta,
    Meter,
    Nav,
    Noscript,
    Object,
    Ol,
    Optgroup,
    Option,
    Output,
    P,
    Param,
    Picture,
    Pre,
    Progress,
    Q,
    Rp,
    Rt,
    Ruby,
    S,
    Samp,
    Script,
    Section,
    Select,
    Small,
    Source,
    Span,
    Strong,
    Style,
    Sub,
    Summary,
    Sup,
    Table,
    Tbody,
    Td,
    Template,
    Textarea,
    Tfoot,
    Th,
    Thead,
    Time,
    Title,
    Tr,
    Track,
    U,
    Ul,
    Var,
    Video,
    Wbr,
}




#[derive(Debug)]
pub struct Div {
    attributes: Vec<(String, String)>,
}


pub fn div_attributes(
    id: String,
    class: Option<String>,
    title: Option<String>,
    style: Option<String>,
    accesskey: Option<String>,
    dir: Option<String>,
    draggable: Option<bool>,
    hidden: Option<bool>,
    spellcheck: Option<bool>,
    tabindex: Option<i32>,
    placeholder: Option<String>,
    href: Option<String>,
    src: Option<String>,
    alt: Option<String>,
    name: Option<String>,
    value: Option<String>,
    r#type: Option<String>, // r#type afin
    checked: Option<bool>,
    disabled: Option<bool>,
    readonly: Option<bool>,
    required: Option<bool>,
    role: Option<String>,
    aria_label: Option<String>,
    aria_hidden: Option<bool>,
    aria_role: Option<String>,
    data: Option<HashMap<String, String>>,
) -> Attributes {
    Attributes {
        id,
        class,
        title,
        style,
        accesskey,
        dir,
        draggable,
        hidden,
        spellcheck,
        tabindex,
        placeholder,
        href,
        src,
        alt,
        name,
        value,
        r#type,
        checked,
        disabled,
        readonly,
        required,
        role,
        aria_label,
        aria_hidden,
        aria_role,
        data,
    }
}


#[derive(Debug)]
pub struct Attributes {
    // Attribut obligatoire
    pub id: String,

    // Attributs globaux optionnels
    pub class: Option<String>,
    pub title: Option<String>,
    pub style: Option<String>,
    pub accesskey: Option<String>,
    pub dir: Option<String>,
    pub draggable: Option<bool>,
    pub hidden: Option<bool>,
    pub spellcheck: Option<bool>,
    pub tabindex: Option<i32>,
    
    // Attributs spécifiques
    pub placeholder: Option<String>,
    pub href: Option<String>,
    pub src: Option<String>,
    pub alt: Option<String>,
    pub name: Option<String>,
    pub value: Option<String>,
    pub r#type: Option<String>, // r#type afin d'échapper la propriété "type" (struct Attributes) qui est un mot réservé en rust
    pub checked: Option<bool>,
    pub disabled: Option<bool>,
    pub readonly: Option<bool>,
    pub required: Option<bool>,
    pub role: Option<String>,

    // Attributs ARIA
    pub aria_label: Option<String>,
    pub aria_hidden: Option<bool>,
    pub aria_role: Option<String>,

    // Attributs data-*
    pub data: Option<HashMap<String, String>>,
}


#[derive(Debug)]
pub struct Html {
    attributes: Vec<(String, String)>,
}

impl Html {
    pub fn new() -> Self {
        Self {
            attributes: Vec::new(),
        }
    }

    pub fn add_attribute(&mut self, key: &str, value: &str) {
        self.attributes.push((key.to_string(), value.to_string()));
    }
}