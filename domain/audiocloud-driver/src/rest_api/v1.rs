use actix_web::error::ErrorNotFound;
use actix_web::{get, post, put, web, Error, Responder};

use audiocloud_api::common::media::{PlayId, RenderId};
use audiocloud_api::instance_driver::{InstanceDriverCommand, InstanceDriverError};
use audiocloud_api::newtypes::FixedInstanceId;
use audiocloud_api::DesiredInstancePlayState;

use crate::drivers::Instances;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(get_parameters)
       .service(get_instances)
       .service(set_parameters)
       .service(set_parameter)
       .service(set_desired_instance_state);
}

#[get("/instances")]
async fn get_instances(instances: web::Data<Instances>) -> impl Responder {
    let rv = instances.get_instances();

    Ok::<_, Error>(web::Json(rv))
}

#[get("/{manufacturer}/{name}/{instance}")]
async fn get_parameters(instances: web::Data<Instances>, path: web::Path<(String, String, String)>) -> impl Responder {
    let instance_id = get_instance_id(path.into_inner());

    let rv = instances.get_parameters(&instance_id).map_err(to_actix_error)?;

    Ok::<_, Error>(web::Json(rv))
}

#[post("/{manufacturer}/{name}/{instance}/parameters")]
async fn set_parameters(instances: web::Data<Instances>,
                        path: web::Path<(String, String, String)>,
                        params: web::Json<serde_json::Value>)
                        -> impl Responder {
    let instance_id = get_instance_id(path.into_inner());

    let rv = instances.set_parameters(&instance_id, params.into_inner())
                      .await
                      .map_err(to_actix_error)?;

    Ok::<_, Error>(web::Json(rv))
}

#[post("/{manufacturer}/{name}/{instance}/parameters/{parameter_id}")]
async fn set_parameter(instances: web::Data<Instances>,
                       path: web::Path<(String, String, String, String)>,
                       value: web::Json<serde_json::Value>)
                       -> impl Responder {
    let (manufacturer, name, instance, parameter_id) = path.into_inner();
    let instance_id = get_instance_id((manufacturer, name, instance));

    let mut values = serde_json::Map::new();
    values.insert(parameter_id, value.into_inner());

    let rv = instances.set_parameters(&instance_id, serde_json::Value::Object(values))
                      .await
                      .map_err(to_actix_error)?;

    Ok::<_, Error>(web::Json(rv))
}

#[put("/{manufacturer}/{name}/{instance}/play_state")]
async fn set_desired_instance_state(instances: web::Data<Instances>,
                                    path: web::Path<(String, String, String)>,
                                    state: web::Json<DesiredInstancePlayState>)
                                    -> impl Responder {
    let instance_id = get_instance_id(path.into_inner());

    let rv = instances.set_desired_play_state(&instance_id, state.into_inner())
                      .await
                      .map_err(to_actix_error)?;

    Ok::<_, Error>(web::Json(rv))
}

fn get_instance_id((manufacturer, name, instance): (String, String, String)) -> FixedInstanceId {
    FixedInstanceId::new(manufacturer, name, instance)
}

fn to_actix_error(error: InstanceDriverError) -> Error {
    use InstanceDriverError::*;

    match &error {
        InstanceNotFound { .. } | ParameterDoesNotExist { .. } => ErrorNotFound(error),
        MediaNotPresent | DriverNotSupported { .. } => actix_web::error::ErrorNotImplemented(error),
        ParametersMalformed { .. } | ReportsMalformed { .. } | ConfigMalformed { .. } => actix_web::error::ErrorBadRequest(error),
        RPC { .. } => actix_web::error::ErrorBadGateway(error),
        _ => actix_web::error::ErrorInternalServerError(error),
    }
}
