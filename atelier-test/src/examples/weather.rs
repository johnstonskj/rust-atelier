use crate::TestCaseModel;
use atelier_core::builder::{
    traits, ArrayBuilder, ListBuilder, MemberBuilder, ModelBuilder, ObjectBuilder,
    OperationBuilder, ResourceBuilder, ServiceBuilder, ShapeTraits, SimpleShapeBuilder,
    StructureBuilder,
};
use atelier_core::error::ErrorSource;
use atelier_core::model::Model;
use atelier_core::Version;
use std::convert::TryInto;

const WEATHER_AS_LINES: &[&str] = &[
    "list::example.weather#CitySummaries",
    "list::example.weather#CitySummaries::member=>example.weather#CitySummary",
    "operation::example.weather#GetCity",
    "operation::example.weather#GetCity::error=>example.weather#NoSuchResource",
    "operation::example.weather#GetCity::input=>example.weather#GetCityInput",
    "operation::example.weather#GetCity::output=>example.weather#GetCityInput",
    "operation::example.weather#GetCity::trait::smithy.api#readonly<={}",
    "operation::example.weather#GetCurrentTime",
    "operation::example.weather#GetCurrentTime::trait::smithy.api#readonly<={}",
    "operation::example.weather#GetForecast",
    "operation::example.weather#GetForecast::input=>example.weather#GetForecastInput",
    "operation::example.weather#GetForecast::output=>example.weather#GetForecastInput",
    "operation::example.weather#GetForecast::trait::smithy.api#readonly<={}",
    "operation::example.weather#ListCities",
    "operation::example.weather#ListCities::input=>example.weather#ListCitiesInput",
    "operation::example.weather#ListCities::output=>example.weather#ListCitiesInput",
    "operation::example.weather#ListCities::trait::smithy.api#paginated<={items}=\"items\"",
    "operation::example.weather#ListCities::trait::smithy.api#readonly<={}",
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
    "structure::example.weather#CityCoordinates::latitude::trait::smithy.api#required<={}",
    "structure::example.weather#CityCoordinates::latitude=>smithy.api#Float",
    "structure::example.weather#CityCoordinates::longitude::trait::smithy.api#required<={}",
    "structure::example.weather#CityCoordinates::longitude=>smithy.api#Float",
    "structure::example.weather#CitySummary",
    "structure::example.weather#CitySummary::cityId::trait::smithy.api#required<={}",
    "structure::example.weather#CitySummary::cityId=>example.weather#CityId",
    "structure::example.weather#CitySummary::name::trait::smithy.api#required<={}",
    "structure::example.weather#CitySummary::name=>smithy.api#String",
    "structure::example.weather#CitySummary::trait::smithy.api#references<=[0]={resource}=\"City\"",
    "structure::example.weather#GetCityInput",
    "structure::example.weather#GetCityInput::cityID::trait::smithy.api#required<={}",
    "structure::example.weather#GetCityInput::cityID=>example.weather#CityId",
    "structure::example.weather#GetCityOutput",
    "structure::example.weather#GetCityOutput::coordinates::trait::smithy.api#required<={}",
    "structure::example.weather#GetCityOutput::coordinates=>example.weather#CityCoordinates",
    "structure::example.weather#GetCityOutput::name::trait::smithy.api#required<={}",
    "structure::example.weather#GetCityOutput::name=>smithy.api#String",
    "structure::example.weather#GetCurrentTimeOutput",
    "structure::example.weather#GetCurrentTimeOutput::time::trait::smithy.api#required<={}",
    "structure::example.weather#GetCurrentTimeOutput::time=>smithy.api#Timestamp",
    "structure::example.weather#GetForecastInput",
    "structure::example.weather#GetForecastInput::cityId::trait::smithy.api#required<={}",
    "structure::example.weather#GetForecastInput::cityId=>example.weather#CityId",
    "structure::example.weather#GetForecastOutput",
    "structure::example.weather#GetForecastOutput::chanceOfRain=>smithy.api#Float",
    "structure::example.weather#ListCitiesInput",
    "structure::example.weather#ListCitiesInput::nextToken=>smithy.api#String",
    "structure::example.weather#ListCitiesInput::pageSize=>smithy.api#Integer",
    "structure::example.weather#ListCitiesOutput",
    "structure::example.weather#ListCitiesOutput::items::trait::smithy.api#required<={}",
    "structure::example.weather#ListCitiesOutput::items=>example.weather#CitySummaries",
    "structure::example.weather#ListCitiesOutput::nextToken=>smithy.api#String",
    "structure::example.weather#NoSuchResource",
    "structure::example.weather#NoSuchResource::resourceType::trait::smithy.api#required<={}",
    "structure::example.weather#NoSuchResource::resourceType=>smithy.api#String",
    "structure::example.weather#NoSuchResource::trait::smithy.api#error<=\"client\"",
];

pub fn make_weather_model() -> TestCaseModel {
    let model: Model = ModelBuilder::new(Version::V10, "example.weather")
        .service(
            ServiceBuilder::new("Weather", "2006-03-01")
                .documentation("Provides weather forecasts.")
                .paginated(Some("nextToken"), Some("nextToken"), None, Some("pageSize"))
                .resource("City")
                .operation("GetCurrentTime")
                .into(),
        )
        .resource(
            ResourceBuilder::new("City")
                .identifier("cityId", "CityId")
                .read("GetCity")
                .list("ListCities")
                .resource("Forecast")
                .into(),
        )
        .resource(
            ResourceBuilder::new("Forecast")
                .identifier("cityId", "CityId")
                .read("GetForecast")
                .into(),
        )
        .simple_shape(
            SimpleShapeBuilder::string("CityId")
                .apply_trait(traits::pattern("^[A-Za-z0-9 ]+$"))
                .into(),
        )
        .operation(
            OperationBuilder::new("GetCity")
                .readonly()
                .input("GetCityInput")
                .output("GetCityOutput")
                .error("NoSuchResource")
                .into(),
        )
        .structure(
            StructureBuilder::new("GetCityInput")
                .add_member(MemberBuilder::new("cityID", "CityId").required().into())
                .into(),
        )
        .structure(
            StructureBuilder::new("GetCityOutput")
                .add_member(MemberBuilder::string("name").required().into())
                .add_member(
                    MemberBuilder::new("coordinates", "CityCoordinates")
                        .required()
                        .into(),
                )
                .into(),
        )
        .structure(
            StructureBuilder::new("CityCoordinates")
                .add_member(MemberBuilder::float("latitude").required().into())
                .add_member(MemberBuilder::float("longitude").required().into())
                .into(),
        )
        .structure(
            StructureBuilder::new("NoSuchResource")
                .error_source(ErrorSource::Client)
                .add_member(MemberBuilder::string("resourceType").required().into())
                .into(),
        )
        .operation(
            OperationBuilder::new("ListCities")
                .paginated(None, None, Some("items"), None)
                .readonly()
                .input("ListCitiesInput")
                .output("ListCitiesOutput")
                .into(),
        )
        .structure(
            StructureBuilder::new("ListCitiesInput")
                .string("nextToken")
                .integer("pageSize")
                .into(),
        )
        .structure(
            StructureBuilder::new("ListCitiesOutput")
                .string("nextToken")
                .add_member(
                    MemberBuilder::new("items", "CitySummaries")
                        .required()
                        .into(),
                )
                .into(),
        )
        .list(ListBuilder::new("CitySummaries", "CitySummary"))
        .structure(
            StructureBuilder::new("CitySummary")
                .apply_trait(traits::references(
                    ArrayBuilder::default()
                        .push(
                            ObjectBuilder::default()
                                .reference("resource", "City")
                                .into(),
                        )
                        .into(),
                ))
                .add_member(MemberBuilder::new("cityId", "CityId").required().into())
                .add_member(MemberBuilder::string("name").required().into())
                .into(),
        )
        .operation(
            OperationBuilder::new("GetCurrentTime")
                .readonly()
                .output("GetCurrentTimeOutput")
                .into(),
        )
        .structure(
            StructureBuilder::new("GetCurrentTimeOutput")
                .add_member(MemberBuilder::timestamp("time").required().into())
                .into(),
        )
        .operation(
            OperationBuilder::new("GetForecast")
                .readonly()
                .input("GetForecastInput")
                .output("GetForecastOutput")
                .into(),
        )
        .structure(
            StructureBuilder::new("GetForecastInput")
                .add_member(MemberBuilder::new("cityId", "CityId").required().into())
                .into(),
        )
        .structure(
            StructureBuilder::new("GetForecastOutput")
                .float("chanceOfRain")
                .into(),
        )
        .try_into()
        .unwrap();
    TestCaseModel {
        model,
        expected_lines: WEATHER_AS_LINES.to_vec(),
    }
}
