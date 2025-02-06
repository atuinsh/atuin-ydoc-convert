use assert_json_diff::assert_json_include;
use atuin_ydoc_convert::convert_to_value;
use serde_json::Value;
use std::fs;

fn assert_json_incl(expected: &Value, actual: &Value) {
    let actual =
        &serde_json::from_str::<Value>(&json_digest::canonical_json(&actual).unwrap()).unwrap();
    let expected =
        &serde_json::from_str::<Value>(&json_digest::canonical_json(&expected).unwrap()).unwrap();

    assert_json_include!(actual: actual, expected: expected);
}

#[test]
fn test_convert_basic_block() {
    let input = r#"
    <blockgroup>
      <blockcontainer backgroundColor="default" id="f6596d68-4414-48f3-b502-eb54c9a00b17" textColor="default">
        <paragraph textAlignment="left">Some text</paragraph>
      </blockcontainer>
    </blockgroup>
    "#;

    let expected: Value = serde_json::from_str(
        r#"
    [
      {
        "id": "f6596d68-4414-48f3-b502-eb54c9a00b17",
        "type": "paragraph",
        "props": {
          "textColor": "default",
          "backgroundColor": "default",
          "textAlignment": "left"
        },
        "content": [{ "type": "text", "text": "Some text", "styles": {} }],
        "children": []
      }
    ]
    "#,
    )
    .unwrap();

    let result = convert_to_value(input.to_string()).unwrap();

    assert_json_incl(&expected, &result);
}

#[test]
fn test_convert_http_block() {
    let input = r#"
    <blockgroup>
      <blockcontainer backgroundColor="default" id="6eb44f4d-36b9-4dbe-9fad-1278774ddd14" textColor="default">
        <http body="" headers="{}" name="HTTP" url="http://google.com/testing things and stuff!!!" verb="PUT"></http>
      </blockcontainer>
    </blockgroup>
    "#;

    // TODO: remove backgroundColor and textColor and content
    let expected: Value = serde_json::from_str(
        r#"
    [
      {
        "id": "6eb44f4d-36b9-4dbe-9fad-1278774ddd14",
        "type": "http",
        "props": {
          "backgroundColor": "default",
          "textColor": "default",
          "name": "HTTP",
          "url": "http://google.com/testing things and stuff!!!",
          "verb": "PUT",
          "body": "",
          "headers": "{}"
        },
        "children": [],
        "content": []
      }
    ]
    "#,
    )
    .unwrap();

    let result = convert_to_value(input.to_string()).unwrap();

    assert_json_incl(&expected, &result);
}

#[test]
fn test_convert_formatted_content() {
    let input = r#"
    <blockgroup>
      <blockcontainer backgroundColor="default" id="350b5292-78b5-4a3a-9ad0-193b208f3411" textColor="default">
        <paragraph textAlignment="left">Here is <bold>some </bold><bold><italic>text</italic></bold>, including <link class="null" href="https://google.com" rel="noopener noreferrer nofollow" target="_blank">a link</link>!</paragraph>
      </blockcontainer>
    </blockgroup>
    "#;

    let expected: Value = serde_json::from_str(
        r#"
    [
      {
        "id": "350b5292-78b5-4a3a-9ad0-193b208f3411",
        "type": "paragraph",
        "props": {
          "textColor": "default",
          "backgroundColor": "default",
          "textAlignment": "left"
        },
        "content": [
          { "type": "text", "text": "Here is ", "styles": {} },
          { "type": "text", "text": "some ", "styles": { "bold": true } },
          { "type": "text", "text": "text", "styles": { "bold": true, "italic": true } },
          { "type": "text", "text": ", including ", "styles": {} },
          {
            "type": "link",
            "href": "https://google.com",
            "rel": "noopener noreferrer nofollow",
            "target": "_blank",
            "styles": {},
            "class": "null",
            "content": [{ "type": "text", "text": "a link", "styles": {} }]
          },
          { "type": "text", "text": "!", "styles": {} }
        ],
        "children": []
      }
    ]
    "#,
    )
    .unwrap();

    let result = convert_to_value(input.to_string()).unwrap();

    assert_json_incl(&expected, &result);
}

#[test]
fn test_convert_nested_block() {
    let input = r#"
    <blockgroup>
      <blockcontainer backgroundColor="default" id="f6596d68-4414-48f3-b502-eb54c9a00b17" textColor="default">
        <bulletlistitem textAlignment="left">One</bulletlistitem>
      </blockcontainer>
      <blockcontainer backgroundColor="default" id="2447823e-192b-4781-8d9e-860de53dab00" textColor="default">
        <bulletlistitem textAlignment="left">Two</bulletlistitem>
        <blockgroup>
          <blockcontainer backgroundColor="default" id="3615c0ef-0de3-495b-96b4-0bb7c488bdd3" textColor="default">
            <bulletlistitem textAlignment="left">Nested</bulletlistitem>
          </blockcontainer>
          <blockcontainer backgroundColor="default" id="63096453-0847-4203-8efa-11cbfc1486b6" textColor="default">
            <bulletlistitem textAlignment="left">Again</bulletlistitem>
          </blockcontainer>
        </blockgroup>
      </blockcontainer>
      <blockcontainer backgroundColor="default" id="526b8f4a-c60b-47a0-9ebd-54c84657b209" textColor="default">
        <bulletlistitem textAlignment="left">Three</bulletlistitem>
      </blockcontainer>
      <blockcontainer backgroundColor="default" id="30bd3f34-0288-43fe-9045-c58b71305e9d" textColor="default">
        <paragraph textAlignment="left"></paragraph>
      </blockcontainer>
    </blockgroup>
    "#;

    let expected: Value = serde_json::from_str(
        r#"
    [
      {
        "id": "f6596d68-4414-48f3-b502-eb54c9a00b17",
        "type": "bulletListItem",
        "props": {
          "textColor": "default",
          "backgroundColor": "default",
          "textAlignment": "left"
        },
        "content": [{ "type": "text", "text": "One", "styles": {} }],
        "children": []
      },
      {
        "id": "2447823e-192b-4781-8d9e-860de53dab00",
        "type": "bulletListItem",
        "props": {
          "textColor": "default",
          "backgroundColor": "default",
          "textAlignment": "left"
        },
        "content": [{ "type": "text", "text": "Two", "styles": {} }],
        "children": [
          {
            "id": "3615c0ef-0de3-495b-96b4-0bb7c488bdd3",
            "type": "bulletListItem",
            "props": {
              "textColor": "default",
              "backgroundColor": "default",
              "textAlignment": "left"
            },
            "content": [{ "type": "text", "text": "Nested", "styles": {} }],
            "children": []
          },
          {
            "id": "63096453-0847-4203-8efa-11cbfc1486b6",
            "type": "bulletListItem",
            "props": {
              "textColor": "default",
              "backgroundColor": "default",
              "textAlignment": "left"
            },
            "content": [{ "type": "text", "text": "Again", "styles": {} }],
            "children": []
          }
        ]
      },
      {
        "id": "526b8f4a-c60b-47a0-9ebd-54c84657b209",
        "type": "bulletListItem",
        "props": {
          "textColor": "default",
          "backgroundColor": "default",
          "textAlignment": "left"
        },
        "content": [{ "type": "text", "text": "Three", "styles": {} }],
        "children": []
      },
      {
        "id": "30bd3f34-0288-43fe-9045-c58b71305e9d",
        "type": "paragraph",
        "props": {
          "textColor": "default",
          "backgroundColor": "default",
          "textAlignment": "left"
        },
        "content": [],
        "children": []
      }
    ]
    "#,
    )
    .unwrap();

    let result = convert_to_value(input.to_string()).unwrap();

    assert_json_incl(&expected, &result);
}

#[test]
fn test_convert_list() {
    let input = r#"
    <blockgroup>
      <blockcontainer backgroundColor="default" id="a3f0e2c9-be18-40bc-8d29-8e26b14c2f7c" textColor="default">
        <numberedlistitem index="1" textAlignment="left">Numbered</numberedlistitem>
        <blockgroup>
          <blockcontainer backgroundColor="default" id="4fd5efc1-f05e-4e70-8ee6-9c75aa218281" textColor="default">
            <numberedlistitem index="1" textAlignment="left">Nested</numberedlistitem>
          </blockcontainer>
          <blockcontainer backgroundColor="default" id="3bd78d63-ce31-4297-9ee3-ef186be02772" textColor="default">
            <numberedlistitem index="2" textAlignment="left">added 2</numberedlistitem>
          </blockcontainer>
          <blockcontainer backgroundColor="default" id="defb2607-7adf-4aff-8ec9-8f31a27e5d70" textColor="default">
            <numberedlistitem index="3" textAlignment="left">List</numberedlistitem>
          </blockcontainer>
          <blockcontainer backgroundColor="default" id="f25cfc0d-11e1-4232-b1af-31a2ab84e0dd" textColor="default">
            <numberedlistitem index="4" textAlignment="left">added 3</numberedlistitem>
          </blockcontainer>
        </blockgroup>
      </blockcontainer>
    </blockgroup>
    "#;

    let expected: Value = serde_json::from_str(
        r#"
    [
      {
        "id": "a3f0e2c9-be18-40bc-8d29-8e26b14c2f7c",
        "type": "numberedListItem",
        "props": {
          "index": "1",
          "textColor": "default",
          "backgroundColor": "default",
          "textAlignment": "left"
        },
        "content": [{ "type": "text", "text": "Numbered", "styles": {} }],
        "children": [
          {
            "id": "4fd5efc1-f05e-4e70-8ee6-9c75aa218281",
            "type": "numberedListItem",
            "props": {
              "index": "1",
              "textColor": "default",
              "backgroundColor": "default",
              "textAlignment": "left"
            },
            "content": [{ "type": "text", "text": "Nested", "styles": {} }],
            "children": []
          },
          {
            "id": "3bd78d63-ce31-4297-9ee3-ef186be02772",
            "type": "numberedListItem",
            "props": {
              "index": "2",
              "textColor": "default",
              "backgroundColor": "default",
              "textAlignment": "left"
            },
            "content": [{ "type": "text", "text": "added 2", "styles": {} }],
            "children": []
          },
          {
            "id": "defb2607-7adf-4aff-8ec9-8f31a27e5d70",
            "type": "numberedListItem",
            "props": {
              "index": "3",
              "textColor": "default",
              "backgroundColor": "default",
              "textAlignment": "left"
            },
            "content": [{ "type": "text", "text": "List", "styles": {} }],
            "children": []
          },
          {
            "id": "f25cfc0d-11e1-4232-b1af-31a2ab84e0dd",
            "type": "numberedListItem",
            "props": {
              "index": "4",
              "textColor": "default",
              "backgroundColor": "default",
              "textAlignment": "left"
            },
            "content": [{ "type": "text", "text": "added 3", "styles": {} }],
            "children": []
          }
        ]
      }
    ]
    "#,
    )
    .unwrap();

    let result = convert_to_value(input.to_string()).unwrap();

    assert_json_incl(&expected, &result);
}

#[test]
fn test_convert_table() {
    let input = r#"
    <blockGroup>
      <blockContainer id="f6596d68-4414-48f3-b502-eb54c9a00b17" textColor="default" backgroundColor="default">
        <table>
          <tableRow>
            <tableCell rowspan="1" colspan="1">
              <tableParagraph>one</tableParagraph>
            </tableCell>
            <tableCell colspan="1" rowspan="1" colwidth="[335]">
              <tableParagraph>two</tableParagraph>
            </tableCell>
            <tableCell rowspan="1" colspan="1">
              <tableParagraph><italic>three</italic></tableParagraph>
            </tableCell>
          </tableRow>
          <tableRow>
            <tableCell colspan="1" rowspan="1">
              <tableParagraph><bold>four</bold></tableParagraph>
            </tableCell>
            <tableCell rowspan="1" colspan="1" colwidth="[335]">
              <tableParagraph>f<bold>iv</bold>e</tableParagraph>
            </tableCell>
            <tableCell rowspan="1" colspan="1">
              <tableParagraph><bold><italic><strike>six</strike></italic></bold></tableParagraph>
            </tableCell>
          </tableRow>
        </table>
      </blockContainer>
    </blockGroup>
    "#;

    let expected: Value = serde_json::from_str(
        r#"
    [
      {
        "id": "f6596d68-4414-48f3-b502-eb54c9a00b17",
        "type": "table",
        "props": { "textColor": "default" },
        "content": {
          "type": "tableContent",
          "columnWidths": [null, 335, null],
          "rows": [
            {
              "cells": [
                [{ "type": "text", "text": "one", "styles": {} }],
                [{ "type": "text", "text": "two", "styles": {} }],
                [{ "type": "text", "text": "three", "styles": { "italic": true } }]
              ]
            },
            {
              "cells": [
                [{ "type": "text", "text": "four", "styles": { "bold": true } }],
                [
                  { "type": "text", "text": "f", "styles": {} },
                  { "type": "text", "text": "iv", "styles": { "bold": true } },
                  { "type": "text", "text": "e", "styles": {} }
                ],
                [
                  {
                    "type": "text",
                    "text": "six",
                    "styles": { "bold": true, "italic": true, "strike": true }
                  }
                ]
              ]
            }
          ]
        },
        "children": []
      }
    ]
    "#,
    )
    .unwrap();

    let result = convert_to_value(input.to_string()).unwrap();

    assert_json_incl(&expected, &result);
}

#[test]
fn test_everything_bagel() {
    let input = fs::read_to_string("tests/fixtures/everything_input.xml").unwrap();
    let expected =
        serde_json::from_str(&fs::read_to_string("tests/fixtures/everything_output.json").unwrap())
            .unwrap();

    let result = convert_to_value(input.to_string()).unwrap();

    assert_json_incl(&expected, &result);
}
