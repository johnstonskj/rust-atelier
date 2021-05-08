use atelier_core::io::lines::make_line_oriented_form;

pub mod common;

const MOTD_LINES: &[&str] = &[
    "operation::example.motd#GetMessage",
    "operation::example.motd#GetMessage::error=>example.motd#BadDateValue",
    "operation::example.motd#GetMessage::input=>example.motd#GetMessageInput",
    "operation::example.motd#GetMessage::output=>example.motd#GetMessageInput",
    "operation::example.motd#GetMessage::trait::smithy.api#readonly",
    "resource::example.motd#Message",
    "resource::example.motd#Message::identifier::date=>example.motd#Date",
    "resource::example.motd#Message::read=>example.motd#GetMessage",
    "service::example.motd#MessageOfTheDay",
    "service::example.motd#MessageOfTheDay::resource=>example.motd#Message",
    "service::example.motd#MessageOfTheDay::trait::smithy.api#documentation<=\"Provides a Message of the day.\"",
    "service::example.motd#MessageOfTheDay::version<=\"2020-06-21\"",
    "string::example.motd#Date",
    "string::example.motd#Date::trait::smithy.api#pattern<=\"^\\\\d\\\\d\\\\d\\\\d\\\\-\\\\d\\\\d-\\\\d\\\\d$\"",
    "structure::example.motd#BadDateValue",
    "structure::example.motd#BadDateValue::errorMessage::trait::smithy.api#required",
    "structure::example.motd#BadDateValue::errorMessage=>smithy.api#String",
    "structure::example.motd#BadDateValue::trait::smithy.api#error<=\"client\"",
    "structure::example.motd#GetMessageInput",
    "structure::example.motd#GetMessageInput::date=>example.motd#Date",
    "structure::example.motd#GetMessageOutput",
    "structure::example.motd#GetMessageOutput::message::trait::smithy.api#required",
    "structure::example.motd#GetMessageOutput::message=>smithy.api#String",

];

const WEATHER_LINES: &[&str] = &[
    "list::example.weather#CitySummaries",
    "list::example.weather#CitySummaries::member=>example.weather#CitySummary",
    "operation::example.weather#GetCity",
    "operation::example.weather#GetCity::error=>example.weather#NoSuchResource",
    "operation::example.weather#GetCity::input=>example.weather#GetCityInput",
    "operation::example.weather#GetCity::output=>example.weather#GetCityInput",
    "operation::example.weather#GetCity::trait::smithy.api#readonly",
    "operation::example.weather#GetCurrentTime",
    "operation::example.weather#GetCurrentTime::trait::smithy.api#readonly",
    "operation::example.weather#GetForecast",
    "operation::example.weather#GetForecast::input=>example.weather#GetForecastInput",
    "operation::example.weather#GetForecast::output=>example.weather#GetForecastInput",
    "operation::example.weather#GetForecast::trait::smithy.api#readonly",
    "operation::example.weather#ListCities",
    "operation::example.weather#ListCities::input=>example.weather#ListCitiesInput",
    "operation::example.weather#ListCities::output=>example.weather#ListCitiesInput",
    "operation::example.weather#ListCities::trait::smithy.api#paginated<={items}=\"items\"",
    "operation::example.weather#ListCities::trait::smithy.api#readonly",
    "resource::example.weather#City",
    "resource::example.weather#City::identifier::cityId=>example.weather#CityId",
    "resource::example.weather#City::list=>example.weather#ListCities",
    "resource::example.weather#City::read=>example.weather#GetCity",
    "resource::example.weather#City::resource=>example.weather#Forecast",
    "resource::example.weather#Forecast",
    "resource::example.weather#Forecast::identifier::cityId=>example.weather#CityId",
    "resource::example.weather#Forecast::read=>example.weather#GetForecast",
    "service::example.weather#Weather",
    "service::example.weather#Weather::operation=>example.weather#GetCurrentTime",
    "service::example.weather#Weather::resource=>example.weather#City",
    "service::example.weather#Weather::trait::smithy.api#documentation<=\"Provides weather forecasts.\"",
    "service::example.weather#Weather::trait::smithy.api#paginated<={inputToken}=\"nextToken\"",
    "service::example.weather#Weather::trait::smithy.api#paginated<={outputToken}=\"nextToken\"",
    "service::example.weather#Weather::trait::smithy.api#paginated<={pageSize}=\"pageSize\"",
    "service::example.weather#Weather::version<=\"2006-03-01\"",
    "string::example.weather#CityId",
    "string::example.weather#CityId::trait::smithy.api#pattern<=\"^[A-Za-z0-9 ]+$\"",
    "structure::example.weather#CityCoordinates",
    "structure::example.weather#CityCoordinates::latitude::trait::smithy.api#required",
    "structure::example.weather#CityCoordinates::latitude=>smithy.api#Float",
    "structure::example.weather#CityCoordinates::longitude::trait::smithy.api#required",
    "structure::example.weather#CityCoordinates::longitude=>smithy.api#Float",
    "structure::example.weather#CitySummary",
    "structure::example.weather#CitySummary::cityId::trait::smithy.api#required",
    "structure::example.weather#CitySummary::cityId=>example.weather#CityId",
    "structure::example.weather#CitySummary::name::trait::smithy.api#required",
    "structure::example.weather#CitySummary::name=>smithy.api#String",
    "structure::example.weather#CitySummary::trait::smithy.api#references<=[0]={resource}=\"City\"",
    "structure::example.weather#GetCityInput",
    "structure::example.weather#GetCityInput::cityID::trait::smithy.api#required",
    "structure::example.weather#GetCityInput::cityID=>example.weather#CityId",
    "structure::example.weather#GetCityOutput",
    "structure::example.weather#GetCityOutput::coordinates::trait::smithy.api#required",
    "structure::example.weather#GetCityOutput::coordinates=>example.weather#CityCoordinates",
    "structure::example.weather#GetCityOutput::name::trait::smithy.api#required",
    "structure::example.weather#GetCityOutput::name=>smithy.api#String",
    "structure::example.weather#GetCurrentTimeOutput",
    "structure::example.weather#GetCurrentTimeOutput::time::trait::smithy.api#required",
    "structure::example.weather#GetCurrentTimeOutput::time=>smithy.api#Timestamp",
    "structure::example.weather#GetForecastInput",
    "structure::example.weather#GetForecastInput::cityId::trait::smithy.api#required",
    "structure::example.weather#GetForecastInput::cityId=>example.weather#CityId",
    "structure::example.weather#GetForecastOutput",
    "structure::example.weather#GetForecastOutput::chanceOfRain=>smithy.api#Float",
    "structure::example.weather#ListCitiesInput",
    "structure::example.weather#ListCitiesInput::nextToken=>smithy.api#String",
    "structure::example.weather#ListCitiesInput::pageSize=>smithy.api#Integer",
    "structure::example.weather#ListCitiesOutput",
    "structure::example.weather#ListCitiesOutput::items::trait::smithy.api#required",
    "structure::example.weather#ListCitiesOutput::items=>example.weather#CitySummaries",
    "structure::example.weather#ListCitiesOutput::nextToken=>smithy.api#String",
    "structure::example.weather#NoSuchResource",
    "structure::example.weather#NoSuchResource::resourceType::trait::smithy.api#required",
    "structure::example.weather#NoSuchResource::resourceType=>smithy.api#String",
    "structure::example.weather#NoSuchResource::trait::smithy.api#error<=\"client\"",
];

#[test]
fn test_motd_to_lines() {
    let model = common::make_message_of_the_day_model();
    let lines = make_line_oriented_form(&model);
    println!("{:#?}", lines);
    assert_eq!(lines, MOTD_LINES.to_vec());
}

#[test]
fn test_weather_to_lines() {
    let model = common::make_weather_model();
    let lines = make_line_oriented_form(&model);
    println!("{:#?}", lines);
    assert_eq!(lines, WEATHER_LINES.to_vec());
}
