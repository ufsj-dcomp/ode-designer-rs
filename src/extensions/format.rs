use nom::{
    branch::alt,
    bytes::complete::{is_a, is_not, tag, take_while},
    character::complete::{alpha1, alphanumeric1, anychar, space0},
    character::complete::{char, space1},
    character::{
        complete::{digit1, multispace0},
        is_digit,
    },
    combinator::rest,
    combinator::{cut, map, opt, value},
    combinator::{map_parser, recognize},
    complete::take,
    error::ParseError,
    multi::{many0, many0_count, many1, separated_list0},
    sequence::{delimited, pair, preceded},
    IResult,
};

#[derive(Debug, PartialEq, Eq)]
enum FormatPart {
    Static(String),
    Dynamic(ArgumentSpecifier),
}

#[derive(Debug, PartialEq, Eq)]
enum ArgumentSpecifier {
    Indexed(usize),
    Named(String),
    All { separator: char },
}

fn parse_ident(input: &str) -> IResult<&str, &str> {
    recognize(pair(
        alt((alpha1, tag("_"))),
        many0_count(alt((alphanumeric1, tag("_")))),
    ))(input)
}

fn parse_dollar_preceded_element(input: &str) -> IResult<&str, FormatPart> {
    alt((
        map(digit1, |numb: &str| {
            FormatPart::Dynamic(ArgumentSpecifier::Indexed(numb.parse().unwrap()))
        }),
        map(parse_ident, |ident: &str| {
            FormatPart::Dynamic(ArgumentSpecifier::Named(ident.to_owned()))
        }),
        map(pair(tag("@"), anychar), |(_at, c)| {
            FormatPart::Dynamic(ArgumentSpecifier::All { separator: c })
        }),
        map(many0(is_not("$ ")), |vc| {
            let mut s = String::from("$");
            vc.into_iter().collect_into(&mut s);
            FormatPart::Static(s)
        }),
    ))(input)
}

fn parse_format(input: &str) -> IResult<&str, Vec<FormatPart>> {
    fn parse_element(input: &str) -> IResult<&str, FormatPart> {
        preceded(
            space0,
            alt((
                preceded(tag("$"), parse_dollar_preceded_element),
                map(many1(is_not("$ ")), |st| {
                    FormatPart::Static(st.into_iter().collect())
                }),
            )),
        )(input)
    }

    many0(parse_element)(input)
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use super::*;

    use FormatPart::{
        Static as S,
        Dynamic as D,
    };

    use ArgumentSpecifier::{
        Indexed as I,
        Named as N,
        All,
    };

    #[rstest]
    #[case(
        "   $1    *    $2",
        &[
            D(I(1)),
            S("*".to_string()),
            D(I(2)),
        ]
    )]
    #[case(
        "cos($1)",
        &[
            S("cos(".to_string()),
            D(I(1)),
            S(")".to_string()),
        ]
    )]
    #[case(
        "Sum = $@+",
        &[
            S("Sum".to_string()),
            S("=".to_string()),
            D(All { separator: '+' }),
        ]
    )]
    #[case(
        "$x ^ $y",
        &[
            D(N("x".to_string())),
            S("^".to_string()),
            D(N("y".to_string())),
        ]
    )]
    #[case(
        "π$r²",
        &[
            S("π".to_string()),
            D(N("r".to_string())),
            S("²".to_string()),
        ]
    )]
    fn test_format_parsing(
        #[case] input: &str,
        #[case] expected_format: &[FormatPart]
    ) {
        let (rest, parsed) = parse_format(input).expect("Parsing failed!");
        
        assert!(rest.is_empty(), "There was a string remainder: '{rest}'");

        assert_eq!(parsed, expected_format);
    }
}