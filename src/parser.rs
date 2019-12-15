
use std::marker::PhantomData;




#[derive(Clone)]
pub struct Location
{
    pub name: String,

    pub line: usize,
    pub column: usize
}

impl Location
{
    pub fn new(name: String) -> Location
    {
        Location { name, line: 1, column: 1 }
    }
}




#[derive(Clone)]
pub struct Token<IdType: PartialEq + Clone>
{
    pub location: Location,
    pub id: IdType,
    pub value: Option<String>
}

impl<IdType: PartialEq + Clone> Token<IdType>
{
    pub fn new(location: &Location, id: IdType) -> Token<IdType>
    {
        Token { location: location.clone(), id, value: None }
    }

    pub fn new_value(location: &Location, id: IdType, value: String) -> Token<IdType>
    {
        Token { location: location.clone(), id, value: Some(value) }
    }
}

impl<IdType: PartialEq + Clone> PartialEq for Token<IdType>
{
    fn eq(&self, other: &Self) -> bool
    {
        self.id == other.id
    }
}



pub trait TokenBuffer<IdType: PartialEq + Clone>
{
    fn get_next(&self) -> Option<Token<IdType>>;
}




pub struct MatchResult<IdType, DataType>
    where IdType: PartialEq + Clone,
          DataType: From<Token<IdType>>
{
    pub is_match: bool,
    pub data: Option<DataType>,
    id_type: PhantomData<IdType>
}

impl<IdType, DataType> MatchResult<IdType, DataType>
    where IdType: PartialEq + Clone,
          DataType: From<Token<IdType>>
{
    pub fn succeeded() -> MatchResult<IdType, DataType>
    {
        MatchResult { is_match: true, data: None, id_type: PhantomData }
    }

    pub fn succeeded_with(data: Option<DataType>) -> MatchResult<IdType, DataType>
    {
        MatchResult { is_match: false, data, id_type: PhantomData }
    }

    pub fn failed() -> MatchResult<IdType, DataType>
    {
        MatchResult { is_match: false, data: None, id_type: PhantomData }
    }
}




pub trait Matcher<IdType, DataType>
    where IdType: PartialEq + Clone,
          DataType: From<Token<IdType>>
{
    fn try_match(&self, buffer: &dyn TokenBuffer<IdType>) -> MatchResult<IdType, DataType>;
}




pub fn expect_token<IdType, DataType>(token: &Token<IdType>) -> Box<dyn Matcher<IdType, DataType>>
    where IdType: 'static + PartialEq + Clone,
          DataType: From<Token<IdType>>
{
    struct TokenMatcher<IdType: PartialEq + Clone>
    {
        expected: Token<IdType>
    }

    impl<IdType, DataType> Matcher<IdType, DataType> for TokenMatcher<IdType>
        where IdType: PartialEq + Clone, DataType: From<Token<IdType>>
    {
        fn try_match(&self, buffer: &dyn TokenBuffer<IdType>) -> MatchResult<IdType, DataType>
        {
            let next = buffer.get_next();
            let mut result = MatchResult::failed();

            if next.is_some()
            {
                let next = next.unwrap();

                if next == self.expected
                {
                    result = MatchResult::succeeded();
                }
            }

            result
        }
    }

    Box::new(TokenMatcher { expected: token.clone() })
}



pub type MatchHanderFn<IdType: PartialEq + Clone, DataType: From<Token<IdType>>> =
    dyn FnMut(Token<IdType>) -> DataType;



pub fn expect_data_token<IdType, DataType, ConverterType>
    (token: Token<IdType>, converter: ConverterType) -> Box<dyn Matcher<IdType, DataType>>
    where IdType: 'static + PartialEq + Clone,
          DataType: From<Token<IdType>>,
          ConverterType: Fn(Token<IdType>) -> DataType
{
    struct DataTokenMatcher<IdType: PartialEq + Clone>
    {
        expected: Token<IdType>
    }

    impl<IdType, DataType> Matcher<IdType, DataType> for DataTokenMatcher<IdType>
        where IdType: 'static + PartialEq + Clone,
              DataType: From<Token<IdType>>
    {
        fn try_match(&self, buffer: &dyn TokenBuffer<IdType>) -> MatchResult<IdType, DataType>
        {
            let next = buffer.get_next();
            let mut result = MatchResult::failed();

            if next.is_some()
            {
                let next = next.unwrap();

                if next == self.expected
                {
                    //result = MatchResult::succeeded_with(Some())
                    let converted = || { converter(next) };
                }
            }

            result
        }
    }

    Box::new(DataTokenMatcher { expected: token.clone() })
}
