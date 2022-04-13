use serde_json::{Map, Value};
use std::any::Any;
use std::sync::{Arc, RwLock};
use webthing::{Thing, Property, Event, Action};

pub struct WotDirectory;

#[allow(unused)]
impl Thing for WotDirectory {
    fn as_thing_description(&self) -> Map<String, Value> {
        todo!()
    }
    fn as_any(&self) -> &dyn Any {
        todo!()
    }
    fn as_mut_any(&mut self) -> &mut dyn Any {
        todo!()
    }
    fn get_href(&self) -> String {
        todo!()
    }
    fn get_href_prefix(&self) -> String {
        todo!()
    }
    fn get_ui_href(&self) -> Option<String> {
        todo!()
    }
    fn set_href_prefix(&mut self, prefix: String) {
        todo!()
    }
    fn set_ui_href(&mut self, href: String) {
        todo!()
    }
    fn get_id(&self) -> String {
        todo!()
    }
    fn get_title(&self) -> String {
        todo!()
    }
    fn get_context(&self) -> Value {
        todo!()
    }
    fn get_type(&self) -> Vec<String> {
        todo!()
    }
    fn get_description(&self) -> String {
        // TODO call as_thing_description, then serialize
        todo!()
    }
    fn get_property_descriptions(&self) -> Map<String, Value> {
        todo!()
    }
    fn get_action_descriptions(&self, action_name: Option<String>) -> Value {
        todo!()
    }
    fn get_event_descriptions(&self, event_name: Option<String>) -> Value {
        todo!()
    }
    fn add_property(&mut self, property: Box<(dyn Property + 'static)>) {
        todo!()
    }
    fn remove_property(&mut self, property_name: &str) {
        todo!()
    }
    fn find_property(&mut self, property_name: &str) -> Option<&mut Box<(dyn Property + 'static)>> {
        todo!()
    }
    fn get_property(&self, property_name: &str) -> Option<Value> {
        todo!()
    }
    fn get_properties(&self) -> Map<String, Value> {
        todo!()
    }
    fn has_property(&self, property_name: &str) -> bool {
        todo!()
    }
    fn get_action(&self, action_name: String, action_id: String) -> Option<Arc<RwLock<Box<(dyn Action + 'static)>>>> {
        todo!()
    }
    fn add_event(&mut self, event: Box<(dyn Event + 'static)>) {
        todo!()
    }
    fn add_available_event(&mut self, name: String, metadata: Map<String, Value>) {
        todo!()
    }
    fn add_action(
        &mut self,
        action: Arc<RwLock<Box<(dyn Action + 'static)>>>,
        input: Option<&Value>,
    ) -> Result<(), &str> {
        todo!()
    }
    fn remove_action(&mut self, action_name: String, action_id: String) -> bool {
        todo!()
    }
    fn add_available_action(&mut self, name: String, metadata: Map<String, Value>) {
        todo!()
    }
    fn add_subscriber(&mut self, ws_id: String) {
        todo!()
    }
    fn remove_subscriber(&mut self, ws_id: String) {
        todo!()
    }
    fn add_event_subscriber(&mut self, name: String, ws_id: String) {
        todo!()
    }
    fn remove_event_subscriber(&mut self, name: String, ws_id: String) {
        todo!()
    }
    fn property_notify(&mut self, name: String, value: Value) {
        todo!()
    }
    fn action_notify(&mut self, action: Map<String, Value>) {
        todo!()
    }
    fn event_notify(&mut self, name: String, event: Map<String, Value>) {
        todo!()
    }
    fn start_action(&mut self, name: String, id: String) {
        todo!()
    }
    fn cancel_action(&mut self, name: String, id: String) {
        todo!()
    }
    fn finish_action(&mut self, name: String, id: String) {
        todo!()
    }
    fn drain_queue(&mut self, ws_id: String) -> Vec<std::vec::Drain<'_, String>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
