use crate::{content_type, CodeGen, Result};

use super::{
    function_code::ClientFunctionCode,
    function_params::FunctionParams,
    new_request_code::{AuthCode, NewRequestCode},
    operation_module::OperationModuleCode,
    request_builder_into_future::RequestBuilderIntoFutureCode,
    request_builder_send::RequestBuilderSendCode,
    request_builder_setter::RequestBuilderSettersCode,
    request_builder_struct::RequestBuilderStructCode,
    response_code::ResponseCode,
    set_request_code::SetRequestCode,
    web_operation_gen::WebOperationGen,
};
pub struct OperationCode {
    pub client_functions: Vec<ClientFunctionCode>,
    pub module_code: Vec<OperationModuleCode>,
}

impl OperationCode {
    // Create code for the web operation
    pub fn new(cg: &CodeGen, operation: &WebOperationGen) -> Result<OperationCode> {
        let parameters = &FunctionParams::new(cg, operation)?;

        let verb = operation.0.verb.clone();
        let auth = AuthCode {};
        let new_request_code = NewRequestCode {
            verb,
            auth,
            path: operation.0.path.clone(),
        };

        // get the content-types from the operation, else the spec, else default to json
        let consumes = operation
            .pick_consumes()
            .unwrap_or_else(|| cg.spec.pick_consumes().unwrap_or(content_type::APPLICATION_JSON))
            .to_string();
        let produces = operation
            .pick_produces()
            .unwrap_or_else(|| cg.spec.pick_produces().unwrap_or(content_type::APPLICATION_JSON))
            .to_string();

        let lro = operation.0.long_running_operation;
        let lro_options = operation.0.long_running_operation_options.clone();

        let request_builder = SetRequestCode::new(operation, parameters, consumes);
        let in_operation_group = operation.0.in_group();
        let client_function_code = ClientFunctionCode::new(operation, parameters, in_operation_group)?;
        let request_builder_struct_code = RequestBuilderStructCode::new(parameters, in_operation_group, lro, lro_options.clone());
        let request_builder_setters_code = RequestBuilderSettersCode::new(parameters);
        let response_code = ResponseCode::new(cg, operation, produces)?;
        let request_builder_send_code = RequestBuilderSendCode::new(new_request_code, request_builder, response_code.clone())?;
        let request_builder_intofuture_code = RequestBuilderIntoFutureCode::new(response_code.clone(), lro, lro_options)?;

        let module_code = OperationModuleCode {
            module_name: operation.function_name()?,
            response_code,
            request_builder_struct_code,
            request_builder_setters_code,
            request_builder_send_code,
            request_builder_intofuture_code,
        };

        Ok(OperationCode {
            client_functions: vec![client_function_code],
            module_code: vec![module_code],
        })
    }
}
