#[derive(Debug)]
#[allow(unused)]
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
    // match kind {
    //     TokenKinds::SELECT        => "SELECT",
    //     TokenKinds::UPDATE        => "UPDATE",
    //     TokenKinds::INSERT        => "INSERT",
    //     TokenKinds::DELETE        => "DELETE",
    //     TokenKinds::CREATE        => "CREATE",
    //     TokenKinds::TABLE         => "TABLE",
    //     TokenKinds::COLUMN        => "COLUMN",
    //     TokenKinds::ALTER         => "ALTER",
    //     TokenKinds::PRIMARY       => "PRIMARY",
    //     TokenKinds::KEY           => "KEY",
    //     TokenKinds::NOT           => "NOT",
    //     TokenKinds::NULL          => "NULL",
    //     TokenKinds::INT           => "INT",
    //     TokenKinds::VARCHAR       => "VARCHAR",
    //     TokenKinds::OPEN_PAREN    => "OPEN_PAREN",
    //     TokenKinds::CLOSE_PAREN   => "CLOSE_PAREN",
    //     TokenKinds::OPEN_BRACKET  => "OPEN_BRACKET",
    //     TokenKinds::CLOSE_BRACKET => "CLOSE_BRACKET",
    //     TokenKinds::SEMI_COLON    => "SEMI_COLON",
    //     TokenKinds::COLON         => "COLON",
    //     TokenKinds::AS            => "AS",
    //     TokenKinds::JOIN          => "JOIN",
    //     TokenKinds::FROM          => "FROM",
    //     TokenKinds::ON            => "ON",
    //     TokenKinds::LEFT          => "LEFT",
    //     TokenKinds::RIGHT         => "RIGHT",
    //     TokenKinds::WHERE         => "WHERE",
    //     TokenKinds::AND           => "AND",
    //     TokenKinds::OR            => "OR",
    //     TokenKinds::IN            => "IN",
    //     TokenKinds::COUNT         => "COUNT",
    //     TokenKinds::SUM           => "SUM",
    //     TokenKinds::AVG           => "AVG",
    //     TokenKinds::ASTRISKS      => "ASTRISKS",
    //     TokenKinds::DOT           => "DOT",
    //     TokenKinds::GROUP         => "GROUP",
    //     TokenKinds::BY            => "BY",
    //     TokenKinds::ORDER         => "ORDER",
    //     TokenKinds::ASC           => "ASC",
    //     TokenKinds::DESC          => "DESC",
    //     TokenKinds::DISTINCT      => "DISTINCT",
    //     TokenKinds::LIMIT         => "LIMIT",
    //     TokenKinds::OFFSET        => "OFFSET",
    //     TokenKinds::HAVING        => "HAVING",
    //     TokenKinds::NUMBER        => "NUMBER",
    //     TokenKinds::PLUS          => "PLUS",
    //     TokenKinds::MINUS         => "MINUS",
    //     TokenKinds::FORMAT        => "FORMAT",
    //     TokenKinds::SINGLE_QUOTE  => "SINGLE_QUOTE",
    //     TokenKinds::DOUBLE_QUOTE  => "DOUBLE_QUOTE",
    //     TokenKinds::STRING        => "STRING",
    //     TokenKinds::SHOW          => "SHOW",
    //     TokenKinds::DATABASES     => "DATABASES",
    //     TokenKinds::TABLES        => "TABLES",
    //     TokenKinds::VIEWES        => "VIEWES",
    //     TokenKinds::DATABASE      => "DATABASE",
    //     TokenKinds::VIEW          => "VIEW",
    //     TokenKinds::PARAMETER     => "PARAMETER"
    // }
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
