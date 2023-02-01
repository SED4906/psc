use nom::{
    IResult,
    combinator::{value, recognize, opt},
    sequence::{pair, delimited},
    bytes::complete::is_not,
    character::complete::{char, digit1, alphanumeric0, alpha1, multispace0}, multi::{many1, many0}, branch::alt,
};

#[derive(Clone, Debug)]
pub enum Expression {
    Comment,
    Number(f32),
    HexString(String),
    Identifier(String),
    SlashIdentifier(String),
    String(String),
    Array(Vec<Expression>),
    Block(Vec<Expression>),
}

fn comment(input: &str) -> IResult<&str, Expression>
{
    value(Expression::Comment, pair(char('%'), is_not("\n\r")))(input)
}

fn number(input: &str) -> IResult<&str, Expression> {
    let (input, number) = recognize(pair(pair(opt(char('-')),digit1),opt(pair(char('.'), digit1))))(input)?;
    let result = number.parse::<f32>();
    match result {
        Ok(output) => Ok((input,Expression::Number(output))),
        Err(_) => Err(nom::Err::Error(nom::error::Error{input,code:nom::error::ErrorKind::Fail}))
    }
}

fn hex_string(input: &str) -> IResult<&str, Expression> {
    let (input, hex) = delimited(char('<'), is_not(">"), char('>'))(input)?;
    Ok((input,Expression::HexString(hex.into())))
}

fn identifier(input: &str) -> IResult<&str, Expression> {
    let (input, id) = recognize(pair(alpha1,alphanumeric0))(input)?;
    Ok((input,Expression::Identifier(id.into())))
}

fn slash_identifier(input: &str) -> IResult<&str, Expression> {
    let (input, _) = char('/')(input)?;
    let (input, id) = recognize(pair(alpha1,alphanumeric0))(input)?;
    Ok((input,Expression::SlashIdentifier(id.into())))
}

fn string(input: &str) -> IResult<&str, Expression> {
    let (input, st) = delimited(char('('), is_not(")"), char(')'))(input)?;
    Ok((input,Expression::String(st.into())))
}

fn array(input: &str) -> IResult<&str, Expression> {
    let (input, arr) = delimited(char('['), many1(expression), char(']'))(input)?;
    Ok((input,Expression::Array(arr)))
}

fn block(input: &str) -> IResult<&str, Expression> {
    let (input, blk) = delimited(char('{'), many1(expression), char('}'))(input)?;
    Ok((input,Expression::Block(blk)))
}

fn expression(input: &str) -> IResult<&str, Expression> {
    let (input,_) = multispace0(input)?;
    alt((comment,number,hex_string,identifier,slash_identifier,string,array,block))(input)
}

pub fn program(input: &str) -> IResult<&str, Vec<Expression>> {
    many0(expression)(input)
}