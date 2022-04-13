use serde_json::json;
use std::sync::{Arc, RwLock};
use webthing::{BaseThing, Thing};

fn get_directory_thing() -> Arc<RwLock<Box<dyn Thing + 'static>>> {
    let mut thing = BaseThing::new(
        "urn:dev:directory-thing".to_string(),
        "Thing Description Directory (TDD)".to_string(),
        Some(vec!["DirectoryDescription".to_string()]),
        Some("A web of things description directory".to_string()),
    );

    // TODO fully build the Thing description

    let registration_metadata = json!({
        "uriVariables": {
            "type": {
                "title": "Event type",
                "type": "string",
                "enum": [
                    "created_td",
                    "updated_td",
                    "deleted_td"
                ]
            },
            "td_id": {
                "title": "Identifier of TD in directory",
                "type": "string"
            },
            "include_changes": {
                "title": "Include TD changes inside event data",
                "type": "boolean"
            }
        },
        "forms": [
            {
                "op": "subscribeevent",
                "href": "/events{?type,td_id,include_changes}",
                "subprotocol": "sse",
                "contentType": "text/event-stream",
                "htv:headers": [
                    {
                        "description": "ID of the last event for reconnection",
                        "htv:fieldName": "Last-Event-ID",
                        "htv:fieldValue": ""
                    }
                ],
                "data": {
                    "oneOf": [
                        {
                            "type": "object",
                            "description": "The schema of event data",
                            "properties": {
                                "td_id": {
                                    "type": "string",
                                    "format": "iri-reference",
                                    "description": "Identifier of TD in directory"
                                }
                            }
                        },
                        {
                            "type": "object",
                            "description": "The schema of create event data including the created TD",
                            "properties": {
                                "td_id": {
                                    "type": "string",
                                    "format": "iri-reference",
                                    "description": "Identifier of TD in directory"
                                },
                                "td": {
                                    "type": "object",
                                    "description": "The created TD in a create event"
                                }
                            }
                        },
                        {
                            "type": "object",
                            "description": "The schema of the update event data including the updates to TD",
                            "properties": {
                                "td_id": {
                                    "type": "string",
                                    "format": "iri-reference",
                                    "description": "Identifier of TD in directory"
                                },
                                "td_updates": {
                                    "type": "object",
                                    "description": "The partial TD composed of modified TD parts in an update event"
                                }
                            }
                        }
                    ]
                },
                "scopes": "notifications"
            }
        ]
    });
    let registration_metadata = registration_metadata.as_object().unwrap().clone();
    thing.add_available_event("registration".to_string(), registration_metadata);

    Arc::new(RwLock::new(Box::new(thing)))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn build_directory_thing() {
        let _thing = get_directory_thing();
    }
}
