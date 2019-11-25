# Block Elements

[Official doc](https://api.slack.com/reference/block-kit/block-elements)

## **Button** ![badge][badge-interactive]

![badge][badge-modal] 
![badge][badge-msg] 
![badge][badge-tabs] 

![badge][badge-actions] 
![badge][badge-section]

```rust
let button: BlockElement = Button(ButtonElement::new(
    "button",
    "action_id"
));

// {
//   "type": "button",
//   "text": {
//     "type": "plain_text",
//     "text": "button"
//   },
//   "action_id": "action_id"
// }
let json = serde_json::to_string_pretty(&button).unwrap();
```

## **Date Picker** ![badge][badge-interactive] 

![badge][badge-modal] 
![badge][badge-msg] 
![badge][badge-tabs]

![badge][badge-section] 
![badge][badge-actions]
![badge][badge-input]

```rust
let date_picker: BlockElement = DatePicker(DatePickerElement::new("action_id"));

// {
//   "type": "datepicker",
//   "action_id": "action_id"
// }
let json = serde_json::to_string_pretty(&date_picker).unwrap();
```

## **Image**

![badge][badge-modal] 
![badge][badge-msg] 
![badge][badge-tabs]

![badge][badge-section] 
![badge][badge-context]

```rust
let image: BlockElement = Image(ImageElement::new("image_url", "alt_text"));

// {
//   "type": "image",
//   "image_url": "image_url",
//   "alt_text": "alt_text"
// }
let json = serde_json::to_string_pretty(&image).unwrap();
```

## **Multi-select Menu** ![badge][badge-interactive] 

![badge][badge-modal] 
![badge][badge-msg] 
![badge][badge-tabs]

![badge][badge-section] 
![badge][badge-actions]
![badge][badge-input]

```rust
let multi: BlockElement = MultiStaticSelectMenu(
    MultiStaticSelectMenuElement::new("placeholder", "action_id").options(
        vec![
            OptionObject::new("text1", "value1"),
            OptionObject::new("text2", "value2"),
        ],
    ),
);

// {
//   "type": "multi_static_select",
//   "placeholder": {
//     "type": "plain_text",
//     "text": "placeholder"
//   },
//   "action_id": "action_id",
//   "options": [
//     {
//       "text": {
//         "type": "plain_text",
//         "text": "text1"
//       },
//       "value": "value1"
//     },
//     {
//       "text": {
//         "type": "plain_text",
//         "text": "text2"
//       },
//       "value": "value2"
//     }
//   ]
// }
let json = serde_json::to_string_pretty(&multi).unwrap();
```

*Comming soon.*

## **Overflow Menu** ![badge][badge-interactive]

![badge][badge-modal] 
![badge][badge-msg] 
![badge][badge-tabs]

![badge][badge-section] 
![badge][badge-actions]

```rust
// Options' count must be in 2 ..= 5
// OverflowMenuOption::TwoOptions, ThreeOptions, FourOptions, FiveOptions are defined.
// They take a fixed-size array.
let overflow: BlockElement = OverflowMenu(OverflowMenuElement::new(
        "action_id",
        TwoOptions([
            OptionObject::new("option1", "value1"),
            OptionObject::new("option2", "value2"),
        ])
    )
);

// {
//   "type": "overflow",
//   "action_id": "action_id",
//   "options": [
//     {
//       "text": {
//         "type": "plain_text",
//         "text": "option1"
//       },
//       "value": "value1"
//     },
//     {
//       "text": {
//         "type": "plain_text",
//         "text": "option2"
//       },
//       "value": "value2"
//     }
//   ]
// }
let json = serde_json::to_string_pretty(&overflow).unwrap();
```

## **Plain-text Input** ![badge][badge-interactive]

![badge][badge-modal] 

![badge][badge-section] 
![badge][badge-actions]
![badge][badge-input]

```rust
let input: BlockElement = PlainTextInputElement {
    plain_text_input: PlainTextInput::new("action_id")
};

// {
//   "type": "plain_text_input",
//   "action_id": "action_id"
// }
let json = serde_json::to_string_pretty(&input).unwrap();
```

## **Radio button group** ![badge][badge-interactive]

![badge][badge-tabs] 

![badge][badge-section] 
![badge][badge-actions]
![badge][badge-input]

*Comming soon.*

## **Select Menus** ![badge][badge-interactive]

![badge][badge-modal] 
![badge][badge-msg] 
![badge][badge-tabs]

![badge][badge-section] 
![badge][badge-actions]
![badge][badge-input]

```rust
let select_menu: BlockElement = StaticSelectMenu(StaticSelectMenuElement::new(
    "placeholder",
    "action_id"
)
    .options(vec![
        OptionObject::new("option1", "value1"),
        OptionObject::new("option2", "value2"),
    ])
);

// {
//   "type": "static_select",
//   "placeholder": {
//     "type": "plain_text",
//     "text": "placeholder"
//   },
//   "action_id": "action_id",
//   "options": [
//     {
//       "text": {
//         "type": "plain_text",
//         "text": "option1"
//       },
//       "value": "value1"
//     },
//     {
//       "text": {
//         "type": "plain_text",
//         "text": "option2"
//       },
//       "value": "value2"
//     }
//   ]
// }
let json = serde_json::to_string_pretty(&select_menu).unwrap();
```


[badge-modal]: https://img.shields.io/badge/surface-modal-DE4E2E
[badge-msg]: https://img.shields.io/badge/surface-Messages-F2C744
[badge-tabs]: https://img.shields.io/badge/surface-tabs-007A5A

[badge-section]: https://img.shields.io/badge/block-Section-573D82
[badge-actions]: https://img.shields.io/badge/block-Actions-573D82
[badge-input]: https://img.shields.io/badge/block-Input-573D82
[badge-context]: https://img.shields.io/badge/block-context-573D82

[badge-interactive]: https://img.shields.io/badge/type-interactive-lightgrey