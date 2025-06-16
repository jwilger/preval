# Select Evaluation Options Interactively

## User Story
As a developer, I want to interactively choose which evaluations to run and their configuration options through the TUI so that I don't need to remember command-line flags and can easily explore different evaluation modes.

## Business Value
- Reduces friction for running evaluations
- Eliminates need to memorize command-line options
- Enables discovery of available evaluation types
- Prevents errors from incorrect command syntax
- Makes the tool more approachable for new users

## Acceptance Criteria
- [ ] Show initial selection screen when launched without arguments
- [ ] List all available evaluation types discovered from the system
- [ ] Allow selection of one, multiple, or all evaluation types
- [ ] Provide choice between Fast mode (3 samples) and Full mode
- [ ] Allow configuration of AWS profile or other credentials
- [ ] Show description of what each evaluation type tests
- [ ] Remember last used settings for convenience
- [ ] Provide clear "Start Evaluation" action

## Technical Notes
- Discover evaluators by scanning for compatible executables
- Read evaluator metadata from a manifest or --info flag
- Store user preferences in config file
- Support keyboard navigation with intuitive shortcuts
- Allow both mouse and keyboard interaction
- Validate selections before starting

## Example Selection Screen
```
┌─── PrEval: Select Evaluation Options ─────────────────────┐
│                                                             │
│  Evaluation Types:                                          │
│  ☑ event_extraction     Extract events from text           │
│  ☐ entity_recognition   Identify people, places, orgs      │
│  ☐ sentiment_analysis   Analyze emotional tone             │
│                                                             │
│  Evaluation Mode:                                           │
│  ○ Fast   (3 samples, ~30 seconds)                        │
│  ● Full   (all samples, ~15 minutes)                      │
│                                                             │
│  Configuration:                                             │
│  AWS Profile: [nh-dev          ] (optional)               │
│  Output Dir:  [./reports       ]                          │
│                                                             │
│  [Tab] Navigate  [Space] Toggle  [Enter] Start  [Q] Quit   │
└─────────────────────────────────────────────────────────────┘
```

## Interaction Flow
1. Launch preval with no arguments
2. Selection screen appears
3. User navigates with Tab between sections
4. Space toggles checkboxes, arrow keys change radio buttons
5. Enter starts evaluation with current selections
6. Screen transitions to progress view

## Definition of Done
- [ ] Selection interface is intuitive and responsive
- [ ] Evaluator discovery works across platforms
- [ ] Settings are remembered between sessions
- [ ] Validation prevents invalid configurations
- [ ] Keyboard navigation works smoothly
- [ ] Review all tests and refactor to eliminate via type constraints where possible
- [ ] Audit and restrict visibility of all code to minimum required scope