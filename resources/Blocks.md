# Blocks

[Official doc](https://api.slack.com/reference/block-kit/blocks)

## **Section**

![badge][badge-modal] 
![badge][badge-msg] 
![badge][badge-tabs]

```rust
let section: Block = Section(SectionBlock::new(
    Markdown("*markdown*".into())
)
    .accessory(Button(ButtonElement::new(
        "text",
        "action_id"
    )))
);

// {
//   "type": "section",
//   "text": {
//     "type": "mrkdwn",
//     "text": "*markdown*"
//   },
//   "accessory": {
//     "type": "button",
//     "text": {
//       "type": "plain_text",
//       "text": "text"
//     },
//     "action_id": "action_id"
//   }
// }
let json = serde_json::to_string_pretty(&section).unwrap();
```

## **Divider** 

![badge][badge-modal] 
![badge][badge-msg] 
![badge][badge-tabs]

```rust
let divider: Block = Divider(DividerBlock::new());

// {
//   "type": "divider"
// }
let json = serde_json::to_string_pretty(&divider).unwrap();
```

## **Image**

![badge][badge-modal] 
![badge][badge-msg] 
![badge][badge-tabs]

```rust
let image: Block = Image(ImageBlock::new("image_url", "alt_text"));

// {
//   "type": "image",
//   "image_url": "image_url",
//   "alt_text": "alt_text"
// }
let json = serde_json::to_string_pretty(&image).unwrap();
```

## **Actions** 

![badge][badge-modal] 
![badge][badge-msg] 
![badge][badge-tabs]

```rust
let actions: Block = Actions(ActionsBlock::new(vec![
        Button(ButtonElement::new("button1", "action_id1")),
        Image(ImageElement::new("image_url", "alt_text"))
    ])
);

// {
//   "type": "actions",
//   "elements": [
//     {
//       "type": "button",
//       "text": {
//         "type": "plain_text",
//         "text": "button1"
//       },
//       "action_id": "action_id1"
//     },
//     {
//       "type": "image",
//       "image_url": "image_url",
//       "alt_text": "alt_text"
//     }
//   ]
// }
let json = serde_json::to_string_pretty(&actions).unwrap();
```

## **Context**

![badge][badge-modal] 
![badge][badge-msg] 
![badge][badge-tabs]

```rust
let context: Block = Context(ContextBlock::new(vec![
    ImageContext(ImageElement::new("image_url", "alt_text")),
    TextContext(Markdown("*markdown*".into()))
]));

// {
//   "type": "context",
//   "elements": [
//     {
//       "type": "image",
//       "image_url": "image_url",
//       "alt_text": "alt_text"
//     },
//     {
//       "type": "mrkdwn",
//       "text": "*markdown*"
//     }
//   ]
// }
let json = serde_json::to_string_pretty(&context).unwrap();
```

## **Input**

![badge][badge-modal] 

*Comming soon.*

## **File**

![badge][badge-msg]
 
*Comming soon.*

[badge-modal]: https://img.shields.io/badge/surface-modal-DE4E2E
[badge-msg]: https://img.shields.io/badge/surface-Messages-F2C744
[badge-tabs]: https://img.shields.io/badge/surface-tabs-007A5A