# User Story Driven Development

- Status: accepted
- Deciders: John Wilger
- Date: 2025-06-15
- Tags: process, methodology, planning

## Context and Problem Statement

We need a systematic approach to planning and tracking development work that keeps us focused on user value while maintaining technical quality.

## Decision Drivers

- Focus on user value
- Clear acceptance criteria
- Trackable progress
- Prevent scope creep
- Support incremental development

## Considered Options

1. Traditional requirements documents
2. User story driven development
3. Task-based development
4. Free-form development

## Decision Outcome

Chosen option: "User story driven development", because it maintains focus on user value while providing structure for development.

### Positive Consequences

- Every feature tied to user value
- Clear definition of done
- Natural increments for development
- Easy to prioritize and re-prioritize
- Good communication tool

### Negative Consequences

- Overhead of writing stories
- Some technical work doesn't fit story format
- Need discipline to maintain

## Story Management Process

### Structure
- Stories in `PLANNING/` directory
- Three states: TODO, DOING, DONE
- Numbered for priority (000100, 000200, etc.)
- One story in DOING at a time

### Story Format
- User story statement
- Business value
- Acceptance criteria
- Technical notes
- Example output/UI

### Workflow
1. Pick highest priority from TODO
2. Move to DOING
3. Implement with TDD
4. Move to DONE when deployed
5. Never have multiple stories in progress

## Links

- Supports [Agile MVP Development Approach](20250615-agile-mvp-development-approach.md)