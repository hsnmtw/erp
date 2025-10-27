#[derive(Debug)]
#[allow(unused)]
#[allow(non_camel_case_types)]
pub enum TokenKinds {
    SELECT      = 0,
    UPDATE         ,
    INSERT         ,
    INTO           ,
    VALUES         ,
    DELETE         ,
    CREATE         ,
    TABLE          ,
    COLUMN         ,
    ALTER          ,
    PRIMARY        ,
    KEY            ,
    NOT            ,
    NULL           ,
    INT            ,
    VARCHAR        ,
    AS             ,
    JOIN           ,
    FROM           ,
    ON             ,
    LEFT           ,
    RIGHT          ,
    WHERE          ,
    AND            ,
    OR             ,
    IN             ,
    COUNT          ,
    SUM            ,
    AVG            ,
    ASTRISKS       ,
    DOT            ,
    GROUP          ,
    BY             ,
    ORDER          ,
    ASC            ,
    DESC           ,
    DISTINCT       ,
    LIMIT          ,
    OFFSET         ,
    SHOW           ,
    DATABASES      ,
    TABLES         ,
    VIEWES         ,
    DATABASE       ,
    VIEW           ,
    HAVING         ,
    FORMAT         ,

    OPEN_PAREN     ,
    CLOSE_PAREN    ,
    OPEN_BRACKET   ,
    CLOSE_BRACKET  ,
    SEMI_COLON     ,
    COLON          ,

    IDENTIFIER     ,

    NUMBER         ,
    PLUS           ,
    DIVIDE         ,
    MINUS          ,
    
    SINGLE_QUOTE   ,
    DOUBLE_QUOTE   ,
    STRING         ,
    PARAMETER      ,
    
    SPACE          ,
    PERIOD         ,
    EQUALS         ,
    NOT_EQUALS     ,
    EOF            ,
}


fn get_token_name (kind: &'static TokenKinds) -> String {
    format!("{:?}", kind)
}



#[allow(unused)]
#[derive(Debug)]
pub struct Token {
    kind: &'static TokenKinds,
    val: String
}
impl Token {    
    pub fn new(kind: &'static TokenKinds, val : String) -> Token {
        Token { kind, val }
    }

    #[allow(unused)]
    pub fn to_string(&self) -> String {
        // match self.kind {
        //     TokenKinds::PARAMETER | TokenKinds::NUMBER | TokenKinds::STRING => format!("{}",self.val),
        //     _ => get_token_name(self.kind)
        // }
        format!("{} ({})", get_token_name(self.kind), self.val)
    }
}
