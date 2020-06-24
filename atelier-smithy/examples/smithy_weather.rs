use atelier_core::error::ErrorSource;
use atelier_core::io::write_model_to_string;
use atelier_core::model::builder::values::{ArrayBuilder, ObjectBuilder};
use atelier_core::model::builder::{
    ListBuilder, MemberBuilder, ModelBuilder, OperationBuilder, ResourceBuilder, ServiceBuilder,
    ShapeBuilder, SimpleShapeBuilder, StructureBuilder, TraitBuilder,
};
use atelier_core::model::{Identifier, Model, ShapeID};
use atelier_smithy::io::SmithyWriter;
use std::str::FromStr;

fn main() {
    let mut writer = SmithyWriter::default();
    let model = make_weather_model();
    let output = write_model_to_string(&mut writer, &model);
    assert!(output.is_ok());
    println!("{}", output.unwrap())
}

fn make_weather_model() -> Model {
    ModelBuilder::new("example.weather")
        .shape(
            ServiceBuilder::new("Weather")
                .documentation("Provides weather forecasts.")
                .paginated(Some("nextToken"), Some("nextToken"), None, Some("pageSize"))
                .version("2006-03-01")
                .resource("City")
                .operation("GetCurrentTime")
                .into(),
        )
        .shape(
            ResourceBuilder::new("City")
                .identifier("cityID", "CityID")
                .read("GetCity")
                .list("ListCities")
                .resource("Forecast")
                .into(),
        )
        .shape(
            ResourceBuilder::new("Forecast")
                .identifier("cityId", "CityId")
                .read("GetForecast")
                .into(),
        )
        .shape(
            SimpleShapeBuilder::string("CityId")
                .add_trait(TraitBuilder::pattern("^[A-Za-z0-9 ]+$").into())
                .into(),
        )
        .shape(
            OperationBuilder::new("GetCity")
                .readonly()
                .input("GetCityInput")
                .output("GetCityOutput")
                .error("NoSuchResource")
                .into(),
        )
        .shape(
            StructureBuilder::new("GetCityInput")
                .add_member(
                    MemberBuilder::new("cityID")
                        .required()
                        .refers_to("CityId")
                        .into(),
                )
                .into(),
        )
        .shape(
            StructureBuilder::new("GetCityOutput")
                .add_member(MemberBuilder::string("name").required().into())
                .add_member(
                    MemberBuilder::new("coordinates")
                        .required()
                        .refers_to("CityCoordinates")
                        .into(),
                )
                .into(),
        )
        .shape(
            StructureBuilder::new("CityCoordinates")
                .add_member(MemberBuilder::float("latitude").required().into())
                .add_member(MemberBuilder::float("longitude").required().into())
                .into(),
        )
        .shape(
            StructureBuilder::new("NoSuchResource")
                .error(ErrorSource::Client)
                .add_member(MemberBuilder::string("resourceType").required().into())
                .into(),
        )
        .shape(
            OperationBuilder::new("ListCities")
                .paginated(None, None, Some("items"), None)
                .readonly()
                .input("ListCitiesInput")
                .output("ListCitiesOutput")
                .into(),
        )
        .shape(
            StructureBuilder::new("ListCitiesInput")
                .string("nextToken")
                .integer("pageSize")
                .into(),
        )
        .shape(
            StructureBuilder::new("ListCitiesOutput")
                .string("nextToken")
                .add_member(
                    MemberBuilder::new("items")
                        .required()
                        .refers_to("CitySummaries")
                        .into(),
                )
                .into(),
        )
        .shape(ListBuilder::new("CitySummaries", "CitySummary").into())
        .shape(
            StructureBuilder::new("CitySummary")
                .add_trait(
                    TraitBuilder::references(
                        ArrayBuilder::default()
                            .push(
                                ObjectBuilder::default()
                                    .reference(
                                        Identifier::from_str("resource").unwrap().into(),
                                        ShapeID::from_str("City").unwrap(),
                                    )
                                    .into(),
                            )
                            .into(),
                    )
                    .into(),
                )
                .add_member(
                    MemberBuilder::new("cityId")
                        .required()
                        .refers_to("CityId")
                        .into(),
                )
                .add_member(MemberBuilder::string("name").required().into())
                .into(),
        )
        .shape(
            OperationBuilder::new("GetCurrentTime")
                .readonly()
                .output("GetCurrentTimeOutput")
                .into(),
        )
        .shape(
            StructureBuilder::new("GetCurrentTimeOutput")
                .add_member(MemberBuilder::timestamp("time").required().into())
                .into(),
        )
        .shape(
            OperationBuilder::new("GetForecast")
                .readonly()
                .input("GetForecastInput")
                .output("GetForecastOutput")
                .into(),
        )
        .shape(
            StructureBuilder::new("GetForecastInput")
                .add_member(
                    MemberBuilder::new("cityId")
                        .required()
                        .refers_to("CityId")
                        .into(),
                )
                .into(),
        )
        .shape(
            StructureBuilder::new("GetForecastOutput")
                .float("chanceOfRain")
                .into(),
        )
        .into()
}
