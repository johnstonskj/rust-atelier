/*!
Provides more complete example models.
*/

#[doc(hidden)]
pub mod motd;
pub use motd::make_message_of_the_day_model;

#[doc(hidden)]
pub mod weather;
pub use weather::make_weather_model;
