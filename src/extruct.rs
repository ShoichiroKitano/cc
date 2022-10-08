use nom::{
    branch::{alt, permutation},
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric1, char, multispace0, multispace1},
    combinator::recognize,
    multi::{many0_count, separated_list0},
    sequence::{delimited, pair, separated_pair},
    IResult,
};

pub fn extruct_argments(input: &str) -> IResult<&str, Vec<(&str, &str)>> {
    delimited(
        char('('),
        delimited(
            multispace0,
            separated_list0(
                permutation((multispace0, char(','), multispace0)),
                separated_pair(extruct_identifier, multispace1, extruct_identifier),
            ),
            multispace0,
        ),
        char(')'),
    )(input)
}

pub fn extruct_identifier(input: &str) -> IResult<&str, &str> {
    recognize(pair(
        alt((alpha1, tag("_"))),
        many0_count(alt((alphanumeric1, tag("_")))),
    ))(input)
}
