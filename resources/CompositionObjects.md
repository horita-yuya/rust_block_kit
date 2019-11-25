# Composition Objects

[Official doc](https://api.slack.com/reference/block-kit/block-elements)

## **[Text](../src/composition/text.rs)**
```rust
let text: Text = Markdown("*markdown* text".into());

// {
//   "type": "mrkdwn",
//   "text": "*markdown* text"
// }
let json = serde_json::to_string_pretty(&text).unwrap();
```

## **[Confirmation dialog](../src/composition/confirmation_dialog.rs)** 
```rust
let dialog = ConfirmationDialog::new(
    "title",
    Markdown("*text*".into()),
    "confirm",
    "deny",
);

//  "title": {
//    "type": "plain_text",
//    "text": "title"
//  },
//  "text": {
//    "type": "mrkdwn",
//    "text": ""*text*"
//  },
//  "confirm": {
//    "type": "plain_text",
//    "text": "confirm"
//  },
//  "deny": {
//    "type": "plain_text",
//    "text": "deny"
//  }
let json = serde_json::to_string_pretty(&dialog).unwrap();
```

## **[Option](../src/composition/option.rs)** 
```rust
let option = OptionObject::new(
    "text",
    "value"
);

// {
//   "text": {
//     "type": "plain_text",
//     "text": "text"
//   },
//   "value": "value"
// }
let json = serde_json::to_string_pretty(&option).unwrap();
```
## **[Option group](../src/composition/option_group.rs)** 
```rust
let option_group = OptionGroup::new(
    "label",
    vec![
        OptionObject::new("text", "value")
    ]
);

/// {
///   "label": {
///     "type": "plain_text",
///     "text": "label"
///   },
///   "options": [
///     {
///       "text": {
///         "type": "plain_text",
///         "text": "text"
///       },
///       "value": "value"
///     }
///   ]
/// }
let json = serde_json::to_string_pretty(&option_group).unwrap();
```