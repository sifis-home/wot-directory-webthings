use serde_json::json;
use std::sync::{Arc, RwLock};
use webthing::{BaseProperty, BaseThing, Thing};

#[allow(unused)]
fn get_directory_thing() -> Arc<RwLock<Box<dyn Thing + 'static>>> {
    let mut thing = BaseThing::new(
        "urn:dev:directory-thing".to_string(),
        "Thing Description Directory (TDD)".to_string(),
        Some(vec!["DirectoryDescription".to_string()]),
        Some("A web of things description directory".to_string()),
    );

    // TODO fully build the Thing description

    let retrieve_td = json!({
        "description": "Retrieve a Thing Description",
        "uriVariables": {
            "id": {
                "title": "Thing Description ID",
                "type": "string",
                "format": "iri-reference"
            }
        },
        "forms": [
            {
                "href": "/td/{id}",
                "htv:methodName": "GET",
                "response": {
                    "description": "Success response",
                    "htv:statusCodeValue": 200,
                    "contentType": "application/td+json"
                },
                "additionalResponses": [
                    {
                        "description": "TD with the given id not found",
                        "contentType": "application/problem+json",
                        "htv:statusCodeValue": 404
                    }
                ],
                "scopes": "read"
            }
        ]
    });
    let retrieve_td = retrieve_td.as_object().unwrap().clone();
    thing.add_property(Box::new(BaseProperty::new(
        "retrieveTD".to_owned(),
        json!(null),
        None,
        Some(retrieve_td),
    )));

    let retrieve_tds = json!({
        "description": "Retrieve all Thing Descriptions",
        "forms": [
            {
                "href": "/td",
                "htv:methodName": "GET",
                "response": {
                    "description": "Success response",
                    "htv:statusCodeValue": 200,
                    "contentType": "application/ld+json",
                },
                "scopes": "readAll"
            }
        ]
    });
    let retrieve_tds = retrieve_tds.as_object().unwrap().clone();
    thing.add_property(Box::new(BaseProperty::new(
        "retrieveTDs".to_owned(),
        json!(null),
        None,
        Some(retrieve_tds),
    )));

    let search_json_path = json!({
        "description": "JSONPath syntactic search",
        "uriVariables": {
            "query": {
                "title": "A valid JSONPath expression",
                "type": "string"
            }
        },
        "forms": [
            {
                "href": "/search/jsonpath?query={query}",
                "htv:methodName": "GET",
                "response": {
                    "description": "Success response",
                    "contentType": "application/json",
                    "htv:statusCodeValue": 200
                },
                "additionalResponses": [
                    {
                        "description": "JSONPath expression not provided or contains syntax errors",
                        "contentType": "application/problem+json",
                        "htv:statusCodeValue": 400
                    }
                ],
                "scopes": "search"
            }
        ]
    });
    let search_json_path = search_json_path.as_object().unwrap().clone();
    thing.add_property(Box::new(BaseProperty::new(
        "searchJSONPath".to_owned(),
        json!(null),
        None,
        Some(search_json_path),
    )));

    let search_xpath = json!({
        "description": "XPath syntactic search",
        "uriVariables": {
            "query": {
                "title": "A valid XPath expression",
                "type": "string"
            }
        },
        "forms": [
            {
                "href": "/search/xpath?query={query}",
                "htv:methodName": "GET",
                "response": {
                    "description": "Success response",
                    "contentType": "application/json",
                    "htv:statusCodeValue": 200
                },
                "additionalResponses": [
                    {
                        "description": "JSONPath expression not provided or contains syntax errors",
                        "contentType": "application/problem+json",
                        "htv:statusCodeValue": 400
                    }
                ],
                "scopes": "search"
            }
        ]
    });
    let search_xpath = search_xpath.as_object().unwrap().clone();
    thing.add_property(Box::new(BaseProperty::new(
        "searchXPath".to_owned(),
        json!(null),
        None,
        Some(search_xpath),
    )));

    let search_sparql = json!({
        "description": "SPARQL semantic search",
        "uriVariables": {
            "query": {
                "title": "A valid SPARQL 1.1. query",
                "type": "string"
            }
        },
        "forms": [
            {
                "href": "/search/sparql?query={query}",
                "htv:methodName": "GET",
                "response": {
                    "description": "Success response",
                    "htv:statusCodeValue": 200
                },
                "additionalResponses": [
                    {
                        "description": "JSONPath expression not provided or contains syntax errors",
                        "contentType": "application/problem+json",
                        "htv:statusCodeValue": 400
                    }
                ],
                "scopes": "search"
            },
            {
                "href": "/search/sparql",
                "htv:methodName": "POST",
                "response": {
                    "description": "Success response",
                    "contentType": "application/json",
                    "htv:statusCodeValue": 200
                },
                "additionalResponses": [
                    {
                        "description": "JSONPath expression not provided or contains syntax errors",
                        "contentType": "application/problem+json",
                        "htv:statusCodeValue": 400
                    }
                ],
                "scopes": "search"
            }
        ]
    });
    let search_sparql = search_sparql.as_object().unwrap().clone();
    thing.add_property(Box::new(BaseProperty::new(
        "searchSPARQL".to_owned(),
        json!(null),
        None,
        Some(search_sparql),
    )));

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
