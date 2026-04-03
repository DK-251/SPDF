# 🎭 Playwright + TypeScript — Complete Mastery Roadmap v2.0
### From Absolute Beginner → Architect Level
**Pace:** 1–2 hours/day | **Approach:** Concept → Analogy → Hands-on → Evaluate → Progress
**Version:** 2.0 — includes Reporting Mastery, Dependency Management, Mobile Testing, Debugging, Interview Prep

---

## 🧭 HOW TO USE THIS ROADMAP WITH GEN AI

Every concept below comes with a **Master Prompt**. Copy it and paste it into your Gen AI (Claude / ChatGPT / Gemini). The AI will:
1. Teach you the concept using the 4Ws (What, Why, When, Where)
2. Give you a real-world analogy
3. Walk you through hands-on step by step
4. Give you a mini exercise
5. Evaluate your answer: ✅ Pass → Next concept | 🔁 Revisit → Different approach

---

### 📌 Progress Tracking Prompt (Use at START of every new session)
```
PLAYWRIGHT MASTERY — SESSION RESUME
=====================================
Phase: [NUMBER & NAME]
Week: [NUMBER]
Day: [NUMBER]
Last concept completed: [CONCEPT NAME]
Last exercise result: [PASS / REVISIT]
Score achieved: [X/Y]
Areas I struggled with: [LIST OR "none"]
Total days completed so far: [NUMBER]
=====================================
Please acknowledge my progress, briefly summarise what I learned last session,
and continue teaching from exactly where I left off.
Do not repeat passed concepts unless I explicitly ask for revision.
```

---

### 🆘 Universal "I Am Stuck" Prompt (Use anytime you're blocked)
```
I am stuck on this concept: [CONCEPT NAME]
Here is what I tried: [DESCRIBE WHAT YOU DID]
Here is the error or confusion: [PASTE ERROR OR DESCRIBE CONFUSION]
I have already attempted: [NUMBER] times

Please help me by:
1. Explaining this concept using a completely DIFFERENT analogy than before
2. Showing me the simplest possible working example (minimum code)
3. Identifying what specific misunderstanding is likely causing my confusion
4. Giving me one tiny step I can successfully complete right now to build momentum
Do NOT repeat the same explanation I already heard. Fresh approach only.
```

---

### 🌍 Real-World Connection Prompt (Use in every phase, even early ones)
```
I just learned: [CONCEPT NAME]
I am in Phase [X] of my Playwright learning journey.
Please show me ONE simple way this concept connects to real test automation,
even if I am not writing Playwright tests yet.
Give me a tiny hands-on task (max 15 minutes) that uses this concept on a real website.
```

---

# 🗺️ ROADMAP OVERVIEW

| Phase | Name | Duration | Goal |
|---|---|---|---|
| 0 | Mindset & Setup | Week 1 | Mental models, tools installed |
| 1 | Web & Internet Fundamentals | Week 2 | How web works, DOM, HTTP |
| 2 | JavaScript Essentials | Week 3–5 | JS core for automation |
| 3 | TypeScript Fundamentals | Week 6–7 | Types, interfaces, OOP in TS |
| 4 | Node.js, npm & Dependency Management | Week 8 | Runtime, packages, versioning |
| 5 | Playwright Core | Week 9–13 | Browser automation fundamentals |
| 6 | Reporting Mastery | Week 14–15 | All reporters, tracing, custom reports |
| 7 | Framework Design & POM | Week 16–18 | Architecture, patterns, fixtures |
| 8 | Advanced Playwright | Week 19–21 | API testing, mocking, mobile, debugging |
| 9 | CI/CD & DevOps Integration | Week 22–24 | GitHub Actions, Docker, pipelines |
| 10 | Real World Projects | Week 25–28 | 3 full projects end-to-end |
| 11 | Architect Level | Week 29–31 | Design, strategy, mentoring |
| 12 | Interview Readiness | Week 32 | Mock interviews, portfolio, career |

---

# 📦 PHASE 0 — MINDSET & ENVIRONMENT SETUP
## 🗓️ Week 1

---

### Day 1 — The Learning Mindset & What is Automation Testing?

#### 📌 Concepts:
- What is software testing and why it exists
- Manual vs Automation testing
- Where Playwright fits in the testing world
- Growth mindset for learning tech

#### 🧠 Master Prompt — Day 1:
```
I am an absolute beginner starting to learn Playwright + TypeScript.
I come from a Java Selenium background but I am treating this as a fresh start.
Today is Day 1.

Please teach me:
1. What is software testing and why it exists (use a quality check in a factory analogy)
2. Difference between Manual and Automation testing (use a day-to-day life analogy)
3. Where Playwright fits in the world of test automation (give the big picture)
4. The right mindset to learn a new tech skill (growth mindset, handling confusion)

For each concept: WHAT / WHY / WHEN / WHERE + real-world analogy + 3-bullet summary.

After teaching, give me a mini exercise (3 simple questions, no coding).
Wait for my answers. Evaluate each one, give a score out of 10.
Score >= 7: PASS — tell me to move to Day 2.
Score < 7: re-explain weak areas using a different analogy, then retest.
```

---

### Day 2 — Installing Your Toolkit

#### 📌 Concepts:
- What is VS Code and why developers use it
- What is Node.js (JS engine outside browser)
- What is npm (the app store for JS packages)
- Installing Node.js, VS Code, verifying installation

#### 🧠 Master Prompt — Day 2:
```
I am learning Playwright + TypeScript. Today is Day 2.

Please teach me how to set up my development environment:
1. What is VS Code — why developers use it over Notepad (use an office desk analogy)
2. What is Node.js — explain like I have never heard of it (use a car engine analogy)
3. What is npm — explain like an app store or Amazon for code packages
4. Step by step installation guide: Node.js LTS + VS Code on Windows/Mac
5. Verifying installation: node -v and npm -v in the terminal

For each: WHAT / WHY / WHEN / WHERE + analogy + summary.

Hands-on task: install both tools, run node -v and npm -v, share the output.
If correct: PASS → Day 3.
If incorrect: troubleshoot step by step.
Also explain: what is a terminal and why developers use it.
```

---

### Day 3 — How the Web Works

#### 📌 Concepts:
- What happens when you type a URL
- Client vs Server
- HTML, CSS, JavaScript roles
- The DOM (Document Object Model)

#### 🧠 Master Prompt — Day 3:
```
I am learning Playwright + TypeScript. Today is Day 3.

Please teach me how the web works — Playwright automates web browsers so this is foundational:
1. What happens when I type www.google.com — step by step (use restaurant order analogy: customer, waiter, kitchen)
2. Client vs Server (use TV remote and TV analogy)
3. HTML, CSS, JavaScript roles (use house building analogy: structure, paint, electricity)
4. The DOM — explain like a family tree of a webpage

For each: WHAT / WHY / WHEN / WHERE + analogy + 3-bullet summary.

Hands-on: open Chrome → go to any website → right-click → Inspect → find 2 HTML elements in DevTools.
Share what you found.
PASS: correctly identifies 2 HTML elements.
⚠️ Important distinction to explain: DOM is NOT the same as HTML source code. Explain the difference.
```

---

### Day 4 — HTTP & Browser DevTools

#### 📌 Concepts:
- What is HTTP/HTTPS
- HTTP Methods (GET, POST, PUT, DELETE)
- Status Codes (200, 404, 500, etc.)
- Browser DevTools — Network tab, Console tab

#### 🧠 Master Prompt — Day 4:
```
I am learning Playwright + TypeScript. Today is Day 4.

Please teach me:
1. HTTP/HTTPS — use a postal letter analogy (request = sending letter, response = receiving reply)
2. HTTP Methods: GET, POST, PUT, DELETE — use a library analogy
3. Status Codes: 200, 201, 301, 400, 401, 403, 404, 500 — with real-life situations for each
4. Browser DevTools: how to use the Network tab and Console tab with a real example

Hands-on: open Chrome → go to https://reqres.in → DevTools → Network tab → reload → find one GET request and its status code.
PASS: correctly identifies a network request and status code.
⚠️ Tip: Status codes are like exam results — 2xx=pass, 3xx=redirect, 4xx=your mistake, 5xx=server's mistake.
```

---

### Day 5 — CSS Selectors & XPath

#### 📌 Concepts:
- What are locators and why they matter
- CSS Selectors (basic to intermediate)
- XPath (basic to intermediate)
- Finding locators using DevTools

#### 🧠 Master Prompt — Day 5:
```
I am learning Playwright + TypeScript. Today is Day 5.

Please teach me how to find elements on a webpage:
1. What are locators — use a house address analogy
2. CSS Selectors: by tag, by id (#), by class (.), by attribute, parent-child (>)
3. XPath: absolute vs relative (why absolute is bad), //tag[@attr='val'], contains(), text(), axes
4. How to use DevTools to copy/test selectors

Hands-on: go to https://www.saucedemo.com → find CSS selector for username input + XPath for login button.
PASS: both selectors are valid and unique.
⚠️ Warn about: auto-generated absolute XPaths and position-based selectors that break easily.
```

---

### Day 6 — Revision Day (Phase 0)

#### 🧠 Master Prompt — Day 6:
```
I am learning Playwright + TypeScript. Today is Day 6 — Phase 0 revision.

Please create a 10-question quiz covering Days 1–5:
- Testing concepts, environment setup, web fundamentals, HTTP, CSS/XPath
- Include 3 questions where I write a CSS selector or XPath

Evaluate my answers. Score out of 10.
Score >= 7: PHASE 0 COMPLETE ✅ — ready for JavaScript
Score < 7: re-teach weak areas with new analogies, retest.

Also ask: what was the hardest concept this week and why?
Use my answer to tailor Phase 1 approach.
```

### Day 7 — Rest & Reflect 🧘
*No learning. Let your brain consolidate. Optional: watch one YouTube video on "how browsers work".*

---

# 📦 PHASE 1 — JAVASCRIPT ESSENTIALS
## 🗓️ Weeks 2–4

---

### Day 8 — Variables, Data Types & Operators

#### 🧠 Master Prompt — Day 8:
```
I am learning Playwright + TypeScript. Today is Day 8 — starting JavaScript.
I have a Java background — please draw comparisons where helpful.

Please teach me:
1. What is JavaScript — history, why it exists, where it runs (browser vs Node.js)
2. Variables: var, let, const — scope differences (var=entire house, let/const=current room analogy)
   Why var is dangerous. When to use let vs const (name tag vs whiteboard analogy)
3. Data types: string, number, boolean, null, undefined — compare to Java equivalents
   Difference between null and undefined (empty plate vs no plate analogy)
4. Operators: arithmetic, comparison (== vs === — CRITICAL gotcha), logical

Hands-on: create variables.js — write 10 lines using let, const, all data types, console.log results. Run: node variables.js
PASS: runs without errors, demonstrates all variable types and data types.
⚠️ Critical: == does type coercion (0 == "0" is TRUE). Always use ===.
```

---

### Day 9 — Functions

#### 🧠 Master Prompt — Day 9:
```
I am learning Playwright + TypeScript. Today is Day 9.

Please teach me JavaScript functions:
1. What is a function — use a recipe analogy (ingredients=parameters, dish=return value)
2. Three ways to write functions: declaration, expression, arrow function — show all three doing the same thing
3. Parameters, return values, default parameters
4. Why arrow functions are preferred in modern JS and Playwright

Java comparison: how does this differ from Java methods?

Hands-on: create functions.js with:
- A function taking a name → returns a greeting
- An arrow function taking two numbers → returns sum
- A function with a default parameter
- Call all and console.log results
PASS: all work, at least one arrow function used.
⚠️ Warn: forgetting return keyword, confusing implicit vs explicit arrow return.
```

---

### Day 10 — Control Flow

#### 🧠 Master Prompt — Day 10:
```
I am learning Playwright + TypeScript. Today is Day 10.

Please teach me:
1. if/else if/else — traffic light analogy
2. switch statements — TV channel remote analogy, when to use over if/else
3. Ternary operator: condition ? valueIfTrue : valueIfFalse
4. Truthy and Falsy values — CRITICAL in JS:
   Falsy: false, 0, "", null, undefined, NaN — everything else is truthy
   Why this matters in automation: if(element) checks

Java comparison: what is the same and what is dangerously different (truthy/falsy doesn't exist in Java).

Hands-on: create controlflow.js with:
- A score → grade function using if/else
- Same using switch
- A ternary checking if a user is logged in
- 3 falsy value demonstrations
PASS: all scenarios work, student demonstrates truthy/falsy understanding.
⚠️ if(0) is FALSE in JS. This trips up Java devs who expect only boolean conditions.
```

---

### Day 11 — Arrays & Loops

#### 🧠 Master Prompt — Day 11:
```
I am learning Playwright + TypeScript. Today is Day 11.

Please teach me Arrays and Loops:
1. Arrays — shopping list analogy, index starts at 0 (ground floor = 0 analogy)
2. Array methods: push/pop (stack of plates), map (currency conversion), filter (email filtering),
   find (first available seat), forEach (calling each person on a list), includes
3. Loops: for, for...of, forEach — when to use which

Java comparison: similar to ArrayList but with more built-in functional methods.

Hands-on: create arrays.js with an array of 5 test case names, then:
- forEach to print each
- filter for names containing "login"
- map to convert all to uppercase
- find first name longer than 10 characters
PASS: all operations work, student uses map, filter, and find.
⚠️ map returns a NEW array. forEach returns undefined.
```

---

### Day 12 — Objects

#### 🧠 Master Prompt — Day 12:
```
I am learning Playwright + TypeScript. Today is Day 12.

Please teach me JavaScript Objects — the most important JS concept:
1. What is an object — profile card analogy (name, age, email = properties)
2. Creating objects, dot notation vs bracket notation
3. Nested objects (address inside a person object)
4. Object methods (functions inside objects)
5. Destructuring — unpacking a suitcase analogy: const { name, age } = person
6. Spread operator: {...obj1, ...obj2}

Java comparison: similar to a class with public fields, or a structured HashMap.

Hands-on: create objects.js with:
- A testUser object with name, email, password, address (nested: city, country)
- Access and print each property
- Destructure name and email
- A getFullInfo() method
- Merge two objects using spread
PASS: all operations work, destructuring correctly used.
⚠️ Objects are reference types — const obj = {} does NOT prevent property changes.
```

---

### Day 13 — Async JavaScript (Most Critical for Playwright)

#### 🧠 Master Prompt — Day 13:
```
I am learning Playwright + TypeScript. Today is Day 13.
THIS IS THE MOST IMPORTANT DAY before touching Playwright.
Playwright is entirely async. If I don't understand this, nothing will make sense.

Please teach me:
1. Sync vs Async — restaurant analogy:
   Sync: waiter stands frozen at kitchen waiting (slow)
   Async: waiter takes other orders while food is prepared (efficient)
2. Callbacks — what they are and why callback hell is a nightmare
3. Promises — 3 states: pending, fulfilled, rejected (job application analogy: applied, hired, rejected)
   .then() and .catch()
4. async/await — the clean modern way Playwright uses:
   async makes a function return a Promise
   await pauses until Promise resolves ("wait for the kitchen")
5. try/catch/finally for error handling

Java comparison: similar to CompletableFuture but cleaner syntax.

Hands-on: create async.js — simulate fetching user data (use setTimeout), write with .then first, then rewrite with async/await, add try/catch.
PASS: async/await and try/catch correctly used.
⚠️ BURN THIS IN: forgetting await before page.click() causes silent failures in Playwright. This is the #1 beginner mistake.
```

---

### Day 14 — Revision (JS Week 1)

#### 🧠 Master Prompt — Day 14:
```
I am learning Playwright + TypeScript. Today is Day 14 — JavaScript Week 1 revision.

Test me on Days 8–13: variables, functions, control flow, arrays, objects, async/await.
Format: 15 questions (theory + code reading + code writing).
Include: 2 questions on === vs ==, 2 on async/await, 1 array methods question, 1 write an async function.

Score >= 11: PASS — move to Week 3 JS.
Score < 11: identify weak topics, re-teach with new analogies, retest weak topics only.

End with: "Your JavaScript foundation strength: [WEAK / DEVELOPING / SOLID]"
```

---

### Day 15 — ES6+ Modern Features

#### 🧠 Master Prompt — Day 15:
```
I am learning Playwright + TypeScript. Today is Day 15.

Please teach me modern ES6+ features used constantly in Playwright:
1. Template literals: `Hello ${name}` — why better than concatenation
2. Optional chaining: user?.address?.city — "only open door if it exists" analogy
3. Nullish coalescing: value ?? "default" — "if empty use backup" analogy
4. Short-circuit: condition && doSomething()
5. Spread operator with arrays and objects
6. Rest parameters: ...args

For each: WHAT / WHY / WHEN / WHERE + analogy + code example.

Hands-on: create es6.js demonstrating all 6 features. Then rewrite an earlier exercise using these.
PASS: correct usage of all 6 features.
⚠️ Optional chaining is critical in Playwright — element?.textContent() prevents crashes when element might not exist.
```

---

### Day 16 — Modules & Imports/Exports

#### 🧠 Master Prompt — Day 16:
```
I am learning Playwright + TypeScript. Today is Day 16.

Please teach me JavaScript Modules:
1. What is a module — toolbox analogy (each tool in its own drawer)
2. Why modules exist (before modules: everything in one file — chaos)
3. Named exports vs default exports
4. Importing syntax
5. How Playwright test files use imports: import { test, expect } from '@playwright/test'

Hands-on: create utils.js with 3 named exported functions, main.js that imports and uses all 3.
PASS: imports work, student understands named vs default export.
⚠️ Common mistake: mixing CommonJS (require) with ES Modules (import). Playwright uses import/export always.
```

---

### Day 17 — Error Handling & Debugging

#### 🧠 Master Prompt — Day 17:
```
I am learning Playwright + TypeScript. Today is Day 17.

Please teach me Error Handling and Debugging:
1. Three error types: Syntax (grammar mistake), Runtime (blocked road), Logic (wrong recipe directions)
2. try/catch/finally — deep dive with multiple examples
3. throw new Error("message") — custom errors
4. Console methods: .log, .error, .warn, .table, .time
5. VS Code debugger: breakpoints, watch variables, step through code

Hands-on:
- A divide function with proper error handling (divide by zero)
- console.table to display an array of objects
- Set a breakpoint in VS Code, step through code, share what you observe

PASS: errors handled gracefully, VS Code debugger used successfully.
⚠️ In Playwright, most errors are async errors. Always use try/catch inside async functions.
```

---

### Day 18 — Classes & OOP in JavaScript

#### 🧠 Master Prompt — Day 18:
```
I am learning Playwright + TypeScript. Today is Day 18.

Please teach me JavaScript Classes — this directly leads to Page Object Model:
1. What is a class — blueprint/house analogy (class=blueprint, object=actual house)
2. Constructor, properties, methods
3. Inheritance with extends — vehicle hierarchy analogy (Vehicle → Car → ElectricCar)
4. The super keyword
5. How POM in Playwright is literally just classes (preview the connection)

Java comparison: almost identical to Java classes! This should feel very familiar.

Hands-on: create a class hierarchy:
- BasePage with constructor accepting page, and navigate(url) method
- LoginPage extends BasePage with login(username, password) method
- Instantiate and call methods
PASS: working class hierarchy with inheritance and super.
⚠️ Foreshadow: every Page Object in Playwright follows exactly this pattern.
```

---

### Day 19 — Closures & Scope

#### 🧠 Master Prompt — Day 19:
```
I am learning Playwright + TypeScript. Today is Day 19.

Please teach me Closures and Scope:
1. Scope types: global, function, block — building floor analogy
2. Scope chain — how JS looks up variables in outer scopes
3. What is a closure — backpack analogy:
   (a function carries surrounding variables in a backpack even after leaving that context)
4. Practical closure: a counter factory function
5. Why closures appear in Playwright test helpers and fixtures

Hands-on: write createCounter() returning object with increment, decrement, getValue. Each call must be independent.
PASS: closure-based counter with independent state.
⚠️ Interview favourite. Understanding closures = advanced JS developer.
```

---

### Day 20 — JavaScript Final Assessment

#### 🧠 Master Prompt — Day 20:
```
I am learning Playwright + TypeScript. Today is Day 20 — JavaScript Final Assessment.

Please give me:
Part 1 — Theory (5 questions)
Part 2 — Code reading: predict output (5 questions)
Part 3 — Code writing (5 questions): async function with error handling, class with inheritance, array operations, closure-based helper

15 questions, 15 points.
Score >= 11: JAVASCRIPT CERTIFIED ✅ — ready for TypeScript
Score < 11: identify gaps, targeted revision, retest.

End with: "JavaScript Mastery Level: [BEGINNER / INTERMEDIATE / ADVANCED]"
"Recommended focus before TypeScript: [LIST]"
```

---

# 📦 PHASE 2 — TYPESCRIPT FUNDAMENTALS
## 🗓️ Weeks 5–6

---

### Day 21 — What is TypeScript & Why It Exists

#### 🧠 Master Prompt — Day 21:
```
I am learning Playwright + TypeScript. Today is Day 21 — starting TypeScript.

Please teach me:
1. What is TypeScript — GPS analogy: JS=driving without GPS, TS=driving with GPS that warns before wrong turns
2. Key differences: static typing, type checking at compile time
3. How TypeScript compiles (transpiles) to JavaScript — translator analogy
4. Installing TypeScript: npm install -g typescript
5. tsconfig.json — settings panel analogy
6. Writing and compiling first .ts file

Java comparison: TypeScript feels much more like Java than JavaScript does. This is your homecoming.

Hands-on: install TypeScript, create hello.ts with typed variables and a typed function, compile with tsc hello.ts, run with node hello.js.
PASS: first TypeScript file compiles and runs successfully.
```

---

### Day 22 — TypeScript Types

#### 🧠 Master Prompt — Day 22:
```
I am learning Playwright + TypeScript. Today is Day 22.

Please teach me the TypeScript type system:
1. Primitive types: string, number, boolean with explicit annotations
2. Arrays: string[], number[], Array<string>
3. Tuples: fixed-length fixed-type arrays (coordinate analogy)
4. Type inference — smart contract analogy (TS figures type automatically)
5. Union types: string | number — door accepting card OR key analogy
6. Type aliases: type UserId = string | number
7. The "any" type — WHAT IT IS AND WHY TO AVOID IT

Java comparison: union types don't exist in Java — this is a TS superpower.

Hands-on: create types.ts — variables of each type, a fully typed function, a union type function. Run with tsc then node.
PASS: zero TypeScript compilation errors, no "any" used.
⚠️ Rule: if you use "any", you are writing JavaScript with extra steps. Avoid it.
```

---

### Day 23 — Interfaces & Type Aliases

#### 🧠 Master Prompt — Day 23:
```
I am learning Playwright + TypeScript. Today is Day 23.

Please teach me Interfaces — heavily used in Playwright framework design:
1. What is an interface — contract/job description analogy (defines what must exist, not how)
2. Defining: interface User { name: string; age: number }
3. Optional properties: ?
4. Readonly properties
5. Extending interfaces
6. Interface vs type alias — when to use which

Java comparison: similar to Java interfaces but more flexible — they describe shape, not behavior.

Hands-on: TestConfig interface with required and optional properties, User interface extended by AdminUser, a function that accepts TestConfig.
PASS: correct interface definitions with extension and optional properties.
```

---

### Day 24 — TypeScript Classes & Generics

#### 🧠 Master Prompt — Day 24:
```
I am learning Playwright + TypeScript. Today is Day 24.

Please teach me TS Classes and Generics:
1. TypeScript classes with type annotations
2. Access modifiers: public, private, protected — office building analogy (lobby, office floors, CEO office)
3. Constructor shorthand: constructor(private name: string)
4. Generics: write reusable code for any type safely — box analogy: Box<T> can hold anything but once decided, type-safe
5. Generic constraints: <T extends string | number>

Java comparison: generics almost identical to Java generics. Access modifiers identical.

Hands-on: generic DataStore<T> class with add, get, getAll methods + a typed BasePage class.
PASS: generics work with at least two different type instantiations.
```

---

### Day 25 — TypeScript for Playwright (Enums, Utility Types, tsconfig)

#### 🧠 Master Prompt — Day 25:
```
I am learning Playwright + TypeScript. Today is Day 25.

Please teach me TypeScript features specific to Playwright:
1. Enums: enum BrowserType { CHROMIUM, FIREFOX, WEBKIT } — traffic light analogy
2. Utility types with testing examples:
   - Partial<T>, Required<T>, Pick<T,K>, Omit<T,K>, Record<K,V>
3. tsconfig.json for Playwright: strict mode, target, module
4. TypeScript path aliases: @pages/, @utils/ — set these up NOW so you use them from Day 1 of Playwright
   (teach this here, not Day 47 — bad habits with relative imports form early)
5. How to read Playwright's type definitions in VS Code (hover over types)

Hands-on: configure path aliases in tsconfig.json and verify they resolve correctly.
PASS: path aliases configured and working.
```

---

### Day 26 — TypeScript Revision & Assessment

#### 🧠 Master Prompt — Day 26:
```
I am learning Playwright + TypeScript. Today is Day 26 — TypeScript assessment.

Test me on everything from Days 21–25:
Part 1 — Theory (5 questions)
Part 2 — Type reading: identify errors in given code (5 questions)
Part 3 — Writing: typed interfaces, classes, generic functions for testing scenarios (5 questions)

Score >= 11/15: TYPESCRIPT CERTIFIED ✅
Score < 11: targeted revision and retest.

Final evaluation: "TypeScript Readiness for Playwright: [NOT READY / ALMOST READY / READY]"
```

---

# 📦 PHASE 3 — NODE.JS, NPM & DEPENDENCY MANAGEMENT
## 🗓️ Week 7

---

### Day 27 — Node.js Deep Dive

#### 🧠 Master Prompt — Day 27:
```
I am learning Playwright + TypeScript. Today is Day 27.

Please teach me Node.js:
1. What is Node.js — industrial kitchen analogy (browser=restaurant kitchen for customers, Node=industrial kitchen for everything else)
2. How Node differs from browser JS (no DOM, no window, but has file system access)
3. The Event Loop — single chef managing multiple dishes analogy
4. Built-in modules: fs (filing cabinet), path (GPS directions), os
5. How Playwright uses Node.js to control browsers

Hands-on: Node script that reads a JSON file with test data, writes results to a new file, uses path module for safe file path building.
PASS: script reads and writes files without errors.
```

---

### Day 28 — npm & Package Management (Deep)

#### 📌 Concepts (expanded):
- npm basics, package.json, scripts
- devDependencies vs dependencies
- package-lock.json
- **npm ci vs npm install** (critical CI difference)
- **Semantic versioning (^, ~, exact)**
- **npm audit for security**
- npx

#### 🧠 Master Prompt — Day 28:
```
I am learning Playwright + TypeScript. Today is Day 28.
This is an EXPANDED dependency management day covering everything you need for real projects.

Please teach me npm thoroughly:
1. What is npm — IKEA analogy (catalog=npm, furniture=packages, ordering=npm install)
2. package.json: every field explained
3. dependencies vs devDependencies — building tools vs the actual building analogy
4. package-lock.json — why it exists (reproducible installs, same versions every time)
5. npm scripts — like ant tasks or Maven goals

CRITICAL ADDITIONS:
6. npm ci vs npm install — THIS IS THE MOST IMPORTANT DISTINCTION FOR CI:
   - npm install: may update package-lock.json (dangerous in CI)
   - npm ci: always uses exact lock file versions, fails if lock file is out of sync
   - Rule: ALWAYS use npm ci in CI pipelines, npm install locally
7. Semantic versioning (semver): what does ^4.1.0 vs ~4.1.0 vs 4.1.0 mean?
   - Use a "how strict are you about the recipe?" analogy
   - ^ = accept new features but not breaking changes
   - ~ = accept only bug fixes
   - exact = must be exactly this version
   - Why a minor version bump can break your Playwright tests
8. npm audit: scanning for security vulnerabilities
   - npm audit → see issues
   - npm audit fix → auto-fix safe updates
   - When to do this (on every project, in every CI pipeline)
9. npx — running packages without installing globally

Java comparison: npm = Maven/Gradle, package.json = pom.xml, node_modules = .m2 repository.

Hands-on:
- Create new project: npm init -y
- Run npm audit on it
- Add a custom script and run it
- Demonstrate difference between ^ and exact versioning in package.json
PASS: student creates project, understands semver, runs audit, uses npm ci vs install correctly.
⚠️ The most common CI pipeline failure: developer used npm install locally, CI uses npm ci, lock file is out of sync → build fails.
```

---

### Day 29 — Playwright Browser Version & Dependency Management

#### 📌 NEW CONCEPT — previously missing

#### 🧠 Master Prompt — Day 29:
```
I am learning Playwright + TypeScript. Today is Day 29.
This is a NEW day specifically on Playwright's own dependency model.

Please teach me:
1. How @playwright/test version is tied to browser versions:
   - Playwright ships with specific browser builds — not the Chrome you have installed
   - Use a "Playwright brings its own ingredients" analogy
2. npx playwright install — installs the browsers Playwright needs
3. npx playwright install --with-deps — also installs OS-level browser dependencies (needed in Linux/Docker/CI)
4. npx playwright install chromium (install just one browser)
5. How to check which browser version a Playwright version uses: npx playwright --version
6. Why pinning @playwright/test version is important for team consistency
7. How to safely upgrade Playwright:
   - Read the changelog for breaking changes
   - Update @playwright/test
   - Run npx playwright install to get new browser binaries
   - Run full test suite
   - Fix any selectors/APIs that changed
8. Peer dependency conflicts when combining Playwright with reporter libraries:
   - What peer dependency warnings mean
   - Using --legacy-peer-deps as a last resort (and why it is risky)
   - npm dedupe to resolve duplicate packages

Hands-on:
- Run npx playwright install in your project
- Check current browser versions: npx playwright --version
- Look at the Playwright GitHub releases page for the latest changelog
PASS: student understands the browser-version coupling and can describe upgrade procedure.
⚠️ Never run npx playwright test without first running npx playwright install — browsers may not be present.
```

---

### Day 30 — Git & Version Control

#### 🧠 Master Prompt — Day 30:
```
I am learning Playwright + TypeScript. Today is Day 30.

Please teach me Git:
1. What is Git — time machine for code with checkpoints (commits)
2. What is GitHub — Google Drive for code
3. Core commands with analogies: init, add, commit, push, pull, branch, checkout
4. .gitignore: what to exclude (node_modules MUST be excluded — explain why)
5. Creating a GitHub repo and pushing first commit

Hands-on:
- Initialize git repo in project folder
- Create .gitignore with node_modules/
- Make 3 commits with meaningful messages
- Create GitHub account and push project
PASS: project visible on GitHub with 3 commits.
```

---

### Day 31 — Foundation Checkpoint

#### 🧠 Master Prompt — Day 31:
```
I am learning Playwright + TypeScript. Today is Day 31 — Foundation Complete Review.

Please assess me on everything from Phases 0–3:
- 5 questions: web/HTTP concepts
- 5 questions: JavaScript (async focus)
- 5 questions: TypeScript (types/interfaces)
- 5 questions: Node/npm/Git/dependency management (include semver and npm ci vs install)

Total 20 questions.
Score >= 14: FOUNDATION CERTIFIED ✅ — ready for Playwright Core
Score < 14: targeted revision plan.

Also ask: "Describe in your own words how you would set up a brand new test automation project from scratch, including installing Playwright, managing dependencies, and pushing to GitHub."
Evaluate this answer — it tests if the big picture mental model is correct.
```

---

# 📦 PHASE 4 — PLAYWRIGHT CORE
## 🗓️ Weeks 8–12

---

### Day 32 — Introduction to Playwright

#### 🧠 Master Prompt — Day 32:
```
I am learning Playwright + TypeScript. Today is Day 32 — FIRST DAY OF PLAYWRIGHT! 🎭

Please teach me:
1. What is Playwright — origin (Microsoft, ex-Puppeteer team), problem it solves
2. Playwright vs Selenium detailed comparison:
   - Architecture (WebDriver protocol vs CDP/BiDi)
   - Speed, reliability, modern features
   - Auto-waiting vs explicit waits
3. Playwright's key features
4. Playwright architecture: browsers, browser contexts, pages
   — theater analogy: theater=browser, show=context, scene=page
5. Step by step: npm init playwright@latest — what gets created and why

Hands-on:
- Run npm init playwright@latest in a new folder
- Explore generated folder structure (explain each file and folder)
- Run: npx playwright test
- Open HTML report: npx playwright show-report
PASS: Playwright installed, tests run, report opened.
⚠️ Key mental model: in Playwright, everything is async. Every action needs await. Every test function is async.
```

---

### Day 33 — Your First Playwright Test

#### 🧠 Master Prompt — Day 33:
```
I am learning Playwright + TypeScript. Today is Day 33.

Please teach me the anatomy of a Playwright test:
1. The test() function: test('name', async ({ page }) => {}) — why async, what is the { page } argument
2. page object — the main browser interaction object
3. Basic actions: page.goto, page.locator, locator.click, locator.fill, locator.textContent
4. expect() assertions: toBeVisible, toHaveText, toHaveTitle

Hands-on: write a test on https://www.saucedemo.com that:
1. Goes to login page
2. Fills username: standard_user, password: secret_sauce
3. Clicks login
4. Asserts a product heading is visible
PASS: test runs green ✅.
⚠️ Every page action needs await. Missing await is the #1 beginner mistake.
```

---

### Day 34 — Playwright Locators (The Modern Way)

#### 🧠 Master Prompt — Day 34:
```
I am learning Playwright + TypeScript. Today is Day 34.

Please teach me Playwright Locators — the right way:
1. Why Playwright recommends user-facing locators over CSS/XPath
2. Priority order (best to worst): getByRole, getByLabel, getByPlaceholder, getByText, getByTestId, CSS, XPath
3. How to use each with examples
4. Locator chaining
5. Playwright Codegen: npx playwright codegen https://saucedemo.com

Hands-on: use Codegen on saucedemo.com, then rewrite your Day 33 test using ONLY getByRole/getByLabel/getByPlaceholder.
PASS: test uses Playwright-recommended locators and passes.
⚠️ Never use auto-generated absolute XPaths from Codegen — only use as starting point.
```

---

### Day 35 — Assertions Deep Dive

#### 🧠 Master Prompt — Day 35:
```
I am learning Playwright + TypeScript. Today is Day 35.

Please teach me Playwright Assertions:
1. Auto-retrying assertions vs non-retrying — "keep checking the oven" vs "check oven once" analogy
2. Important matchers: toBeVisible/Hidden, toHaveText/ContainText, toHaveValue, toHaveURL, toHaveTitle, toBeEnabled/Disabled, toBeChecked, toHaveCount
3. Negating: .not.
4. Soft assertions: expect.soft() — log mistake but keep going analogy
5. Custom messages: expect(locator, 'should show error message').toBeVisible()

Hands-on: extend saucedemo test with 5 different assertion types, 1 soft assertion, 1 negative assertion.
PASS: all assertions make logical sense, test runs green.
```

---

### Day 36 — Hooks & Test Organization

#### 🧠 Master Prompt — Day 36:
```
I am learning Playwright + TypeScript. Today is Day 36.

Please teach me Playwright hooks and test structure:
1. Hooks: beforeAll (open restaurant), afterAll (close restaurant), beforeEach (fresh table), afterEach (clean table)
2. test.describe — chapter in a book analogy
3. Nested describes
4. Modifiers: test.skip, test.only (⚠️ dangerous if committed), test.fixme

Hands-on: organize saucedemo tests into a describe "Login Feature" with beforeEach navigating to login page, 3 tests: valid login, invalid password, empty fields.
PASS: all 3 tests pass with proper structure.
```

---

### Day 37 — Playwright Configuration

#### 🧠 Master Prompt — Day 37:
```
I am learning Playwright + TypeScript. Today is Day 37.

Please teach me playwright.config.ts in full:
1. Overall structure
2. Global settings: testDir, timeout (exam time limit analogy), retries (second chance analogy), workers, baseURL
3. use block: headless mode (robot working in dark), viewport, video, screenshot, trace
4. projects: multi-browser runs (chromium, firefox, webkit), device emulation
5. reporter: list, html, dot, json, junit — when to use each
6. Combining multiple reporters: html + junit for CI

Hands-on: configure playwright.config.ts with:
- baseURL for saucedemo
- 1 retry on failure
- Run on chromium and firefox
- Screenshot on failure
- HTML reporter
PASS: tests run on 2 browsers with correct config.
```

---

### Day 38 — Waiting Strategies & Timeouts

#### 🧠 Master Prompt — Day 38:
```
I am learning Playwright + TypeScript. Today is Day 38.

Please teach me Playwright's waiting strategy:
1. Auto-waiting: checks visible, stable, enabled, not obscured — patient assistant analogy
2. What Playwright auto-waits for on each action
3. When auto-waiting is not enough:
   - waitForURL, waitForLoadState, waitForSelector, waitForResponse
4. Custom timeout on specific action: locator.click({ timeout: 5000 })
5. Selenium anti-patterns to UNLEARN: Thread.sleep, explicit WebDriverWait

Hands-on: write a test handling navigation — use waitForURL after an action that causes navigation.
PASS: test handles timing correctly, no hardcoded sleeps.
⚠️ NEVER use page.waitForTimeout() in production tests. It is a code smell.
```

---

### Day 39 — Screenshots, Videos & Trace Viewer (Foundation)

#### 🧠 Master Prompt — Day 39:
```
I am learning Playwright + TypeScript. Today is Day 39.

Please teach me Playwright's visual evidence features:
1. Screenshots: page.screenshot(), locator.screenshot(), fullPage option
2. Videos: configuring in playwright.config.ts — on, off, retain-on-failure, on-first-retry
3. Trace Viewer introduction — flight black box recorder analogy:
   - What a trace contains: actions, screenshots, network, console, source
   - Enabling traces in config: on, off, retain-on-failure
   - Generating: npx playwright test --trace on
   - Opening: npx playwright show-trace trace.zip
4. What each tab in Trace Viewer shows:
   - Actions tab: every step with before/after screenshots
   - Network tab: all requests made during the test
   - Console tab: all console messages
   - Source tab: which line of code ran at each step
5. Debugging modes: --headed, --debug, PWDEBUG=1

Hands-on:
- Enable traces for failing tests in config
- Intentionally break one test
- Open Trace Viewer and identify the failure point using the Actions tab
PASS: student opens Trace Viewer and correctly identifies failure step with evidence from network and console tabs.
```

---

### Day 40 — Trace Viewer Deep Mastery

#### 📌 NEW DAY — previously missing

#### 🧠 Master Prompt — Day 40:
```
I am learning Playwright + TypeScript. Today is Day 40 — Trace Viewer deep mastery.

Yesterday I learned the basics of Trace Viewer. Today I master it as a debugging tool.

Please teach me:
1. Reading the network waterfall in traces:
   - How to identify slow API calls during a test
   - How to spot failed network requests that caused test failures
   - Correlating a UI action with its triggered API call
2. Using the source tab to find exactly which line of test code ran
3. Comparing two traces side by side:
   - A passing trace vs a failing trace — what is different?
4. Using trace to diagnose flaky tests:
   - What does a flaky test look like in a trace?
   - Common patterns: element appearing late, race condition, animation blocking click
5. Attaching traces as artifacts in CI — so teammates can debug CI failures locally
6. Trace file size management: when to use retain-on-failure vs always-on

Hands-on:
- Write a test that makes at least 2 API calls visible in network tab
- Generate a trace
- Open trace and answer: which API call took the longest? What did the console log?
- Deliberately introduce a race condition in a test, trace it, and identify the cause
PASS: student correctly reads network waterfall, identifies race condition from trace evidence.
```

---

### Day 41 — Multiple Pages, Tabs & Frames

#### 🧠 Master Prompt — Day 41:
```
I am learning Playwright + TypeScript. Today is Day 41.

Please teach me advanced browser interactions:
1. New tabs: context.waitForEvent('page') — expecting a visitor to arrive analogy
2. Popups (alert, confirm, dialog): page.on('dialog', handler), dialog.accept/dismiss
3. iframes: TV within a TV analogy, page.frameLocator(selector)
   Common pitfall: locators don't work across iframe boundaries without frameLocator
4. Multiple browser contexts: isolated browser session (incognito tabs) for multi-user scenarios

Hands-on: write a test that clicks a link opening a new tab, switches to it, asserts something, closes and returns.
PASS: test correctly handles new tab and returns to original.
```

---

### Day 42 — Mobile & Device Emulation

#### 📌 NEW DAY — previously missing

#### 🧠 Master Prompt — Day 42:
```
I am learning Playwright + TypeScript. Today is Day 42 — Mobile Testing.
This was NOT in the original roadmap but is expected in most QA roles today.

Please teach me:
1. What is device emulation — "Playwright pretends to be a phone" analogy
   (changes viewport size, user agent, touch capability, device pixel ratio)
2. Using Playwright's built-in device descriptors:
   import { devices } from '@playwright/test'
   use: devices['iPhone 14']
   use: devices['Pixel 7']
3. Configuring mobile projects in playwright.config.ts:
   { name: 'Mobile Chrome', use: { ...devices['Pixel 7'] } }
4. Testing responsive layouts: asserting elements visible/hidden at mobile viewport
5. Touch events: tap vs click, swipe gestures
6. Geolocation emulation: page.setGeolocation({ latitude, longitude })
7. Network condition emulation: slow 3G, offline mode
8. When real device testing is needed vs emulation (limitations of emulation)

Hands-on:
- Add a Mobile Chrome project in playwright.config.ts using Pixel 7
- Run your saucedemo tests on the mobile project
- Add a test that asserts a navigation menu is hidden on mobile (hamburger menu)
PASS: mobile project runs, tests pass on mobile viewport, student understands emulation vs real device.
⚠️ Emulation is NOT the same as running on a real device. It does not test real browser rendering engines on real hardware. For that you need BrowserStack or Sauce Labs.
```

---

### Day 43 — Playwright Core Revision & Checkpoint

#### 🧠 Master Prompt — Day 43:
```
I am learning Playwright + TypeScript. Today is Day 43 — Playwright Core Checkpoint.

Please assess me on Days 32–42:
- Architecture, setup, tests
- Locators, assertions, hooks
- Configuration, waiting strategies
- Screenshots, Trace Viewer (basic + deep), debugging
- Multi-tab, popups, iframes, mobile emulation

Format: 15 questions + 2 code writing tasks:
1. Write a complete test file: describe, hooks, 3 tests, proper locators and assertions
2. Write a playwright.config.ts for multi-browser + mobile + retries + HTML reporter

PASS (70%+): PLAYWRIGHT CORE CERTIFIED ✅ → Move to Reporting Mastery
FAIL: targeted revision.
```

---

# 📦 PHASE 5 — REPORTING MASTERY
## 🗓️ Weeks 13–14
*This is now a dedicated phase — reporting is a core professional skill, not an afterthought.*

---

### Day 44 — Built-in Reporters Deep Dive

#### 🧠 Master Prompt — Day 44:
```
I am learning Playwright + TypeScript. Today is Day 44 — starting Reporting Mastery.

Please teach me all of Playwright's built-in reporters in depth:
1. HTML Reporter: the flagship reporter
   - How to read it: test suites, individual test results, retry history
   - Navigating screenshots, videos, and trace links in the report
   - Filtering by passed/failed/flaky
   - How to serve it locally vs host it
2. List reporter: what it shows in terminal, when to use (local dev)
3. Dot reporter: minimal CI output, one dot per test
4. JSON reporter: structured data for custom processing
5. JUnit XML reporter: standard format used by Jenkins, Azure DevOps, GitHub Actions to display results natively
6. Combining reporters in playwright.config.ts: reporter: [['html'], ['junit', { outputFile: 'results.xml' }]]

Hands-on:
- Configure both HTML and JUnit reporters
- Run your test suite
- Open the HTML report and navigate to a failed test
- Find the trace link, screenshot, and error message in the report
PASS: student can navigate HTML report fluently and locate evidence for any failed test.
```

---

### Day 45 — Console & Network Log Capture in Reports

#### 📌 NEW DAY — previously missing

#### 🧠 Master Prompt — Day 45:
```
I am learning Playwright + TypeScript. Today is Day 45 — capturing rich evidence in reports.

Please teach me how to capture and attach additional evidence to test reports:
1. Capturing console messages during tests:
   page.on('console', msg => console.log(msg.type(), msg.text()))
   - Why this matters: JS errors in the browser are often the ROOT CAUSE of test failures
   - How to store console messages and attach to report on failure
2. Capturing failed network requests:
   page.on('requestfailed', request => {})
   - How to detect and log API failures during tests
3. testInfo.attach() — attaching arbitrary evidence to test results:
   - Attaching API response body as a file
   - Attaching a downloaded file
   - Attaching a custom log string
   - Attaching an additional screenshot at a specific step
4. test.info().annotations — adding custom annotations to test reports:
   - testInfo.annotations.push({ type: 'issue', description: 'JIRA-123' })
5. Structured logging with test steps: test.step() for report readability
   - GPS turn-by-turn analogy — makes reports readable for non-technical stakeholders

Hands-on:
- Add console message capture to a test — attach messages to report on failure
- Use testInfo.attach() to attach an API response body
- Add test.step() annotations to all your tests
- Run and view in HTML report — confirm steps and attachments are visible
PASS: report shows step-level detail, console logs captured, attachment visible on failure.
```

---

### Day 46 — Allure Reporter Deep Mastery

#### 🧠 Master Prompt — Day 46:
```
I am learning Playwright + TypeScript. Today is Day 46 — Allure Reporter deep mastery.

Please teach me Allure Reporter properly (not just installation):
1. What is Allure and why companies use it over the built-in HTML reporter:
   - Rich categorization, historical trends, defect classification, beautiful UI
2. Installing: npm install --save-dev allure-playwright allure-commandline
3. Configuration in playwright.config.ts
4. Running tests and generating: npx allure generate ./allure-results --clean -o allure-report
5. Opening: npx allure open allure-report
6. Allure-specific features:
   - @allure.label() for suite hierarchy and custom labels
   - Setting severity: allure.label('severity', 'critical')
   - Linking to JIRA: allure.link('https://jira.../PROJ-123', 'JIRA-123')
   - Adding description: allure.description('This test verifies...')
   - Attaching screenshots to specific steps
7. Understanding Allure report sections:
   - Overview: summary stats
   - Suites: test hierarchy
   - Behaviors: grouped by feature
   - Categories: defect categories (product bugs vs test issues)
   - Timeline: which tests ran when and on which worker
8. Categories configuration: allure/categories.json — defining custom defect categories

Hands-on:
- Set up Allure reporter on your saucedemo project
- Add severity labels and JIRA links to 3 tests
- Generate and open Allure report
- Navigate to Suites, Behaviors, and Categories tabs
PASS: Allure report generated with custom labels, JIRA links visible in report.
```

---

### Day 47 — Extent Reports Integration

#### 📌 NEW DAY — previously missing

#### 🧠 Master Prompt — Day 47:
```
I am learning Playwright + TypeScript. Today is Day 47 — Extent Reports.
This is very popular in enterprise Java QA shops migrating to Playwright.

Please teach me Extent Reports with Playwright:
1. What is Extent Reports — why it is popular in enterprise QA (Java background familiarity)
2. Available options for Playwright + Extent:
   Option A: playwright-html-reporter (Extent-style HTML)
   Option B: @playwright/test custom reporter implementing the Reporter interface
   Option C: extent-playwright (community package)
3. Installing: npm install playwright-html-reporter
4. Configuration in playwright.config.ts:
   reporter: [['playwright-html-reporter', { outputFolder: 'extent-report', ... }]]
5. What Extent reports show: dashboard, categories, timeline, test steps
6. Customizing: adding environment information, system info, author details
7. Extent vs Allure — when to choose which:
   - Allure: better for open source, complex analytics, historical trends
   - Extent: better for teams coming from Java, simpler setup, customizable HTML

Hands-on:
- Install playwright-html-reporter
- Configure it in playwright.config.ts alongside the built-in HTML reporter
- Run tests and open the Extent-style report
- Compare it to the Allure report
PASS: Extent-style report generated and student can articulate when to choose it over Allure.
```

---

### Day 48 — Custom Reporter Creation

#### 📌 NEW DAY — previously missing (architect-level skill)

#### 🧠 Master Prompt — Day 48:
```
I am learning Playwright + TypeScript. Today is Day 48 — building a custom reporter.
This is an ARCHITECT-LEVEL skill. Most people never do this. It separates seniors from architects.

Please teach me:
1. The Playwright Reporter interface — the contract your custom reporter must implement:
   - onBegin(config, suite): runs before any test — use for setup
   - onTestBegin(test, result): runs before each test
   - onTestEnd(test, result): runs after each test — most used hook
   - onEnd(result): runs after all tests — use for final report generation
   - onStepBegin / onStepEnd: for step-level tracking
2. Why you might build a custom reporter:
   - Sending results to a Slack channel in real time
   - Writing results to a database
   - Triggering a JIRA ticket when a test fails
   - Custom dashboard format for your organisation
3. Creating a minimal custom reporter class in TypeScript
4. Registering it in playwright.config.ts
5. Accessing test metadata: test.title, test.location, result.status, result.duration, result.error

Hands-on: build a custom reporter that:
- Prints a summary line to the terminal after each test: [PASS/FAIL] test name (duration ms)
- At the end, writes a summary JSON file: total, passed, failed, duration
PASS: custom reporter works, JSON summary file generated after test run.
⚠️ This is exactly the skill needed to integrate Playwright with any internal dashboarding or ticketing system.
```

---

### Day 49 — ReportPortal & Merging Sharded Reports

#### 📌 NEW DAY — previously missing

#### 🧠 Master Prompt — Day 49:
```
I am learning Playwright + TypeScript. Today is Day 49 — enterprise reporting and report merging.

Please teach me two important topics:

PART 1 — ReportPortal (enterprise real-time reporting):
1. What is ReportPortal — "mission control dashboard" analogy: see all test results in real time across teams
2. Why enterprises use it: AI defect analysis, historical trends, team dashboards
3. Installing: @reportportal/agent-js-playwright
4. Configuration: endpoint, project, API key
5. What it shows that Allure doesn't: real-time streaming results, cross-project analytics

PART 2 — Merging reports from parallel/sharded CI runs:
1. The problem: when running tests across 4 CI shards, each shard generates its own report
2. Merging HTML reports: npx playwright merge-reports --reporter html ./all-reports/
3. Merging Allure results: just combine allure-results folders before running allure generate
4. Merging JUnit XML: use junit-merge CLI tool
5. Setting up GitHub Actions to collect reports from all shards and merge them

Hands-on:
- Simulate 2 shards by running tests with --shard=1/2 and --shard=2/2
- Merge the resulting reports using npx playwright merge-reports
- Verify the merged report shows results from both shards
PASS: merged report shows all tests from both shards correctly.
```

---

### Day 50 — Test Trend Analysis & Reporting in CI

#### 🧠 Master Prompt — Day 50:
```
I am learning Playwright + TypeScript. Today is Day 50 — test health over time.

Please teach me:
1. Publishing HTML reports to GitHub Pages automatically after each run:
   - Exact GitHub Actions YAML steps for gh-pages branch setup
   - How to access the live report URL after each pipeline run
2. Linking test reports in PR comments using GitHub Actions bot (actions/github-script)
3. JUnit XML in CI: how Jenkins, GitHub Actions, Azure DevOps display test results natively
4. Tracking flakiness over time:
   - What is a flaky test: passes sometimes, fails sometimes (use a faulty light switch analogy)
   - How to identify flakiness: retries in Playwright config, tracking which tests retry most
   - Flaky test quarantine strategy: tag flaky tests, run in isolation, fix before re-enabling
5. Pass rate trend — manually tracking test health via JSON reporter + simple script

Hands-on:
- Set up GitHub Actions to publish HTML report to GitHub Pages
- Add a step that posts a comment on PRs with the report link
PASS: report published to GitHub Pages and PR comment contains report link.
```

---

# 📦 PHASE 6 — FRAMEWORK DESIGN & PAGE OBJECT MODEL
## 🗓️ Weeks 15–17

---

### Day 51 — Why Framework Design Matters

#### 🧠 Master Prompt — Day 51:
```
I am learning Playwright + TypeScript. Today is Day 51 — starting Framework Design.

Please teach me WHY framework design matters before HOW:
1. Test script vs Test framework — single recipe vs full restaurant kitchen analogy
2. The maintenance nightmare: UI changes, 50 tests break
3. DRY principle: Don't Repeat Yourself
4. SRP: Single Responsibility Principle
5. What makes a production-ready framework: maintainable, scalable, readable, reliable
6. Overview of the framework we will build together

Set the stage: describe the real-world e-commerce test project we will build step by step.
PASS: student can articulate why design matters and what makes a framework good vs bad.
```

---

### Day 52 — Page Object Model in TypeScript

#### 🧠 Master Prompt — Day 52:
```
I am learning Playwright + TypeScript. Today is Day 52.

Please teach me Page Object Model:
1. What is POM — TV remote analogy:
   Without POM: reaching inside TV to press circuits directly
   With POM: using a remote (Page Object) to interact with the TV (webpage)
2. Anatomy of a Page Object: constructor accepts page, locators as private properties, public methods
3. BasePage class: shared functionality (navigate, waitForLoad)
4. How to connect Page Objects to test files
5. Folder structure: pages/ folder, one file per page

Hands-on: create BasePage + LoginPage (extending BasePage) + a test using only LoginPage methods.
PASS: test uses ONLY Page Object methods, zero raw locators in test file.
⚠️ Golden rule: if you see CSS selectors in a test file, the design is broken.
```

---

### Day 53 — Advanced POM Patterns

#### 🧠 Master Prompt — Day 53:
```
I am learning Playwright + TypeScript. Today is Day 53.

Please teach me advanced POM patterns:
1. Component Objects: reusable UI components (navigation bar, footer, modal) used by multiple Page Objects
2. Fluent Page Object: methods return Page Objects for chaining
   loginPage.login(user) → returns InventoryPage
3. TypeScript interfaces to define Page Object contracts
4. Factory pattern for creating Page Objects

Hands-on: NavigationBar component + LoginPage → InventoryPage fluent chain + a test that chains login → add to cart → checkout.
PASS: fluent chain works, test reads like a user journey.
```

---

### Day 54 — Test Data Management

#### 🧠 Master Prompt — Day 54:
```
I am learning Playwright + TypeScript. Today is Day 54.

Please teach me Test Data Management:
1. Why hardcoded test data is dangerous (changing phone number in 100 places analogy)
2. Static test data: JSON files in fixtures/ folder
3. Dynamic data with @faker-js/faker: generate random names, emails, addresses
4. .env files, dotenv package, process.env for environment-specific data
5. Data factory pattern: functions creating test data objects

Hands-on:
- Create testdata/ folder with users.json
- Create a UserFactory using faker
- Write a test using both static and dynamic user data
- Use .env for baseURL
PASS: no hardcoded values in test or page object files.
⚠️ Never commit .env files to Git. Add to .gitignore immediately.
```

---

### Day 55 — Playwright Fixtures (The Superpower)

#### 🧠 Master Prompt — Day 55:
```
I am learning Playwright + TypeScript. Today is Day 55.

Please teach me Playwright Fixtures:
1. What are fixtures — movie prop department analogy (prepares what the actor needs before each scene)
2. Built-in fixtures: page, browser, browserContext, request
3. Why fixtures > beforeEach: composable, scoped, self-teardown
4. Creating custom fixtures: extend test with your own
5. A loggedInPage fixture — every test starts already logged in
6. Fixture scope: 'test' (fresh each test) vs 'worker' (shared across tests)

Hands-on: create loggedInPage fixture + authenticatedUser fixture, rewrite tests to use them.
PASS: tests use custom fixtures, no login code duplicated in test files.
```

---

### Day 56 — Test Tagging & Selective Execution Strategy

#### 📌 EXPANDED DAY — was one line before, now a full day

#### 🧠 Master Prompt — Day 56:
```
I am learning Playwright + TypeScript. Today is Day 56 — test tagging strategy.
This is critical for large test suites and CI pipelines.

Please teach me a complete tagging strategy:
1. What are tags and why they exist (filing cabinet with color-coded folders analogy)
2. Playwright's tag syntax: test('login works @smoke @critical', ...)
3. Running tagged subsets: npx playwright test --grep @smoke
4. A professional tagging taxonomy for enterprise projects:
   BY PRIORITY: @smoke (5-10 critical tests, run in < 2 min), @regression (full suite), @critical
   BY FEATURE: @login, @checkout, @search, @api
   BY ENVIRONMENT: @staging-only, @prod-safe
   BY STATUS: @flaky (quarantined tests — run in isolation), @wip (work in progress)
5. Excluding tags: --grep-invert @flaky
6. Combining tags: --grep "@smoke|@critical"
7. Tag strategy in playwright.config.ts: different projects running different tag subsets
8. Tag-based CI pipeline strategy:
   - On every commit: @smoke only (fast feedback, 2 min)
   - On every PR: @regression (full suite)
   - On schedule (nightly): all tests including @flaky in isolation

Hands-on:
- Tag all your existing tests with at least 2 appropriate tags each
- Create a CI config that runs only @smoke tests on push
- Run @smoke subset and verify it is significantly faster than full suite
PASS: tagging strategy applied, @smoke run completes in under 2 minutes.
```

---

### Day 57 — Framework Folder Structure & Best Practices

#### 🧠 Master Prompt — Day 57:
```
I am learning Playwright + TypeScript. Today is Day 57.

Please teach me the ideal production-ready folder structure:

project-root/
├── tests/               (test files by feature)
├── pages/               (page object classes)
├── components/          (reusable UI components)
├── fixtures/            (custom Playwright fixtures)
├── testdata/            (static test data JSON)
├── utils/               (helper functions)
├── config/              (environment configs)
├── reports/             (generated reports — gitignored)
├── .env
├── playwright.config.ts
├── package.json
├── tsconfig.json
└── README.md

For each folder: what goes in it, why it is separate, naming conventions.
Also: path aliases in tsconfig.json so we can use @pages/LoginPage instead of relative paths.
How to write a good README.

Hands-on: restructure entire saucedemo project to match this structure with path aliases working.
PASS: project follows structure, path aliases work.
```

---

### Day 58 — Framework Phase Review & Mini Project

#### 🧠 Master Prompt — Day 58:
```
I am learning Playwright + TypeScript. Today is Day 58 — Framework Phase Review.

Mini Project Assessment: build a complete mini-framework for https://www.saucedemo.com:
- LoginPage, InventoryPage, CartPage, CheckoutPage (POM)
- loggedIn fixture
- Test data from JSON + Faker
- At least 5 test cases: login, add to cart, checkout flow
- Proper folder structure with path aliases
- HTML + Allure reporting with test.step() annotations
- Test tags: @smoke and @regression

Evaluation criteria (10 points):
- No raw locators in test files (2pts)
- No hardcoded data (2pts)
- Fixtures used instead of beforeEach (2pts)
- Tests read like user stories (2pts)
- Both reports generate successfully (2pts)

Score >= 7: FRAMEWORK DESIGN CERTIFIED ✅
```

---

# 📦 PHASE 7 — ADVANCED PLAYWRIGHT
## 🗓️ Weeks 18–20

---

### Day 59 — API Testing with Playwright

#### 🧠 Master Prompt — Day 59:
```
I am learning Playwright + TypeScript. Today is Day 59 — API Testing.

Please teach me:
1. What is API testing — restaurant kitchen inspection analogy:
   UI testing = checking the dining room, API testing = inspecting the kitchen directly
2. Playwright's APIRequestContext: get, post, put, delete
3. Validating API responses: status(), json(), expect(response).toBeOK()
4. Using https://reqres.in as practice API
5. API + UI hybrid tests: use API to set up state, UI to verify (faster than UI-only setup)

Hands-on: API tests for reqres.in — GET /users, POST /user, PUT /users/2, DELETE /users/2.
PASS: all 4 API tests pass with proper assertions.
```

---

### Day 60 — Network Interception & Mocking

#### 🧠 Master Prompt — Day 60:
```
I am learning Playwright + TypeScript. Today is Day 60.

Please teach me Network Interception:
1. page.route() — security guard analogy (you control what goes in and out)
2. Mocking responses: route.fulfill({ status: 200, body: JSON.stringify(mockData) })
3. Blocking requests: route.abort() — block ads, tracking scripts
4. Modifying requests: change headers/body before reaching server
5. Waiting for requests: page.waitForRequest(), page.waitForResponse()
6. HAR files: record and replay network traffic

Hands-on: intercept a GET API call, return mocked response, assert UI renders mocked data.
PASS: mock intercept works, UI reflects mocked response.
⚠️ Powerful: mock a "server error" response to test how UI handles failures.
```

---

### Day 61 — Authentication Strategies

#### 🧠 Master Prompt — Day 61:
```
I am learning Playwright + TypeScript. Today is Day 61.

Please teach me Playwright authentication strategies:
1. The problem: logging in before every test is slow
2. storageState: save cookies + localStorage after login — save game checkpoint analogy
3. Global setup file: authenticate once, all tests reuse it
4. Multiple auth roles: admin, regular user, guest — separate state files
5. test.use({ storageState: 'auth.json' }) for specific tests
6. When NOT to reuse auth state (tests verifying the login flow itself)

Hands-on:
- Create global setup that logs in and saves storageState
- Remove login from all test files
- Measure time difference
PASS: tests run without logging in each time.
```

---

### Day 62 — Visual Testing & Accessibility Testing

#### 🧠 Master Prompt — Day 62:
```
I am learning Playwright + TypeScript. Today is Day 62.

Please teach me Visual and Accessibility testing:

VISUAL:
1. What is visual regression — spot the difference game analogy
2. toHaveScreenshot(): captures baseline, compares future runs pixel by pixel
3. Updating snapshots: --update-snapshots
4. Configuring comparison threshold
5. Element-level vs full-page screenshots

ACCESSIBILITY:
1. What is accessibility (a11y) — why it matters legally and ethically
2. WCAG guidelines overview
3. @axe-core/playwright: automated a11y checks
4. AxeBuilder.analyze() and asserting zero violations

Hands-on: visual test for saucedemo home page + accessibility scan.
PASS: screenshot comparison works, axe scan runs.
```

---

### Day 63 — Debugging & Troubleshooting Mastery

#### 📌 NEW DAY — previously missing (critical real-world skill)

#### 🧠 Master Prompt — Day 63:
```
I am learning Playwright + TypeScript. Today is Day 63 — Debugging Mastery.
This is a DAILY skill. Every automation engineer debugs more than they write.

Please teach me:
1. The debugging mindset: systematic elimination vs random guessing
   Use a doctor diagnosis analogy: symptoms → narrow causes → test hypothesis → confirm

2. Playwright Inspector: the interactive debugger
   - PWDEBUG=1 npx playwright test: opens the inspector
   - Step through each action one by one
   - Inspect elements, try locators interactively
   - Why this is better than adding console.log everywhere

3. --headed mode: watching the browser run in real time
   - When to use: when you suspect timing or UI rendering issues

4. Diagnosing flaky tests — the most important debugging skill:
   Flaky = passes sometimes, fails sometimes (faulty light switch analogy)
   Common root causes:
   - Race condition: test acts before UI is ready (fix: better locators, use waitFor)
   - Shared state between tests (fix: proper test isolation)
   - Environment-specific timing (fix: investigate CI vs local differences using traces)
   - Element covered/obscured (fix: scroll into view, wait for element to be stable)
   - Animation interfering with click (fix: waitForFunction to check animation done)

5. CDP (Chrome DevTools Protocol) in Playwright:
   - page.evaluate(() => ...) — run JavaScript in the browser context
   - page.exposeFunction('myFunc', handler) — expose Node.js function to browser JS
   - cdpSession: accessing raw CDP for advanced scenarios (performance metrics, etc.)

6. Performance metrics via CDP:
   - Capturing page load timings: page.goto() returns timing info
   - Using CDP to get Web Vitals: FCP, LCP, CLS
   - When to use this vs Lighthouse CI

Hands-on:
- Take a flaky test (introduce one deliberately with a race condition)
- Use Playwright Inspector to step through it
- Use the trace to identify the root cause
- Fix the race condition properly (no waitForTimeout)
PASS: student correctly identifies and fixes the root cause of the flaky test without using sleep.
```

---

### Day 64 — Parallel Execution & Test Optimization

#### 🧠 Master Prompt — Day 64:
```
I am learning Playwright + TypeScript. Today is Day 64.

Please teach me Parallel Execution and Optimization:
1. How Playwright parallelizes: workers run files in parallel — multiple checkout lanes analogy
2. fullyParallel mode: tests within a file also run in parallel
3. Why test isolation is CRITICAL for parallel execution
4. Sharding: split suite across multiple CI machines
5. Strategies to reduce test time:
   - API setup instead of UI setup
   - storageState for auth
   - Tag-based subset runs
6. Flaky test root causes and fixes (deeper than Day 63 — focus on parallel-specific issues)

Hands-on:
- Set fullyParallel: true
- Run suite and observe parallel execution
- Run only @smoke subset
- Measure time improvement
PASS: parallel execution observed, tagged subset significantly faster.
```

---

# 📦 PHASE 8 — CI/CD & DEVOPS INTEGRATION
## 🗓️ Weeks 21–23

---

### Day 65 — Introduction to CI/CD

#### 🧠 Master Prompt — Day 65:
```
I am learning Playwright + TypeScript. Today is Day 65.

Please teach me CI/CD fundamentals:
1. CI (Continuous Integration) — factory assembly line analogy
2. CD (Continuous Delivery) — every passing build ready to ship
3. Why tests MUST run in CI
4. The typical pipeline: Code push → Install deps (npm ci) → Build → Run tests → Generate report → Notify
5. Overview: GitHub Actions (we use this), Jenkins, GitLab CI, Azure DevOps
6. YAML format basics

PASS: student explains CI/CD in their own words with an analogy.
```

---

### Day 66 — GitHub Actions for Playwright

#### 🧠 Master Prompt — Day 66:
```
I am learning Playwright + TypeScript. Today is Day 66.

Please teach me GitHub Actions with Playwright:
1. Workflow file structure: .github/workflows/playwright.yml
2. The official Playwright GitHub Actions workflow — walk through every line
3. Key steps: checkout, setup-node, npm ci (NOT npm install — explain why), playwright install --with-deps, playwright test, upload-artifact
4. Triggers: on push to main, on pull request
5. Viewing results in GitHub Actions UI

Hands-on: create workflow file, push to GitHub, watch pipeline run, download HTML report artifact.
PASS: pipeline runs successfully, report downloadable.
⚠️ Always use npm ci in pipelines, not npm install.
```

---

### Day 67 — Secrets, Environments & Security in CI

#### 🧠 Master Prompt — Day 67:
```
I am learning Playwright + TypeScript. Today is Day 67.

Please teach me secrets and security in CI:
1. Why never hardcode credentials — bank vault analogy for GitHub Secrets
2. GitHub Secrets: storing sensitive values, accessing in workflow: ${{ secrets.MY_SECRET }}
3. Passing secrets to Node.js as environment variables
4. Multiple environments: dev, staging, production — workflow_dispatch inputs
5. npm audit in CI pipeline — automatically catch security vulnerabilities on every build
   Add as a CI step: npm audit --audit-level=high (fail pipeline if high severity found)
6. Dependency update strategy in CI:
   - Dependabot: auto-create PRs for outdated packages
   - How to configure Dependabot for npm in .github/dependabot.yml

Hands-on:
- Store test credentials as GitHub Secrets
- Update workflow to use secrets
- Add npm audit step to pipeline
- (Stretch) configure Dependabot for npm updates
PASS: no hardcoded credentials, npm audit runs in pipeline.
```

---

### Day 68 — Docker for Test Automation

#### 🧠 Master Prompt — Day 68:
```
I am learning Playwright + TypeScript. Today is Day 68.

Please teach me Docker for test automation:
1. What is Docker — shipping container analogy (app + all dependencies, runs anywhere)
2. Why Docker solves "works on my machine" problem
3. Playwright's official Docker image: mcr.microsoft.com/playwright
4. Creating a Dockerfile for your test project
5. Building and running Docker container locally
6. Using Docker in GitHub Actions pipeline
7. docker-compose for local development

Hands-on:
- Pull Playwright Docker image
- Run tests inside container
- Update GitHub Actions to use Playwright Docker image
PASS: tests run successfully inside Docker.
```

---

### Day 69 — CI Reporting, Notifications & Monorepo Dependencies

#### 📌 EXPANDED to cover monorepo and npm link

#### 🧠 Master Prompt — Day 69:
```
I am learning Playwright + TypeScript. Today is Day 69 — advanced CI topics.

Please teach me:

PART 1 — CI Reporting & Notifications:
1. Publishing HTML reports to GitHub Pages with exact YAML steps
2. PR comment bot with report link using actions/github-script
3. Slack notifications on failure

PART 2 — Monorepo Dependency Management (architect-level):
1. What is a monorepo — one Git repo containing multiple packages (use an apartment building analogy: one building, many units)
2. npm workspaces: sharing common test utilities across multiple test packages
   package.json: { "workspaces": ["packages/*"] }
3. When you would use this: large organisations with multiple products/test suites sharing base helpers
4. npm link: develop and test a shared utility package locally before publishing
   Use case: building an internal shared test library that multiple teams consume

PART 3 — Dependency update strategy:
1. npm outdated: see all out-of-date packages
2. npm update: update within semver constraints
3. Safe Playwright version upgrade procedure:
   - Check Playwright GitHub releases for breaking changes
   - Update @playwright/test in package.json
   - Run npm install
   - Run npx playwright install
   - Run full test suite
   - Fix any broken selectors or API changes

Hands-on:
- Configure Slack notification in GitHub Actions for test failures
- Run npm outdated on your project and identify any outdated packages
PASS: Slack notification working, student can describe a safe Playwright upgrade procedure.
```

---

# 📦 PHASE 9 — REAL WORLD PROJECTS
## 🗓️ Weeks 24–28

---

### Project 1 (Days 70–76) — E-Commerce Test Suite

#### 🧠 Master Prompt — Project 1:
```
I am learning Playwright + TypeScript. Starting Real World Project 1.

Project: Complete Test Automation Framework for https://www.saucedemo.com

Guide me step by step to build this from scratch. Cover:
1. Authentication: valid login, invalid, locked out, logout
2. Product Catalog: listing, sorting, product details
3. Shopping Cart: add, remove, cart count validation
4. Checkout Flow: complete checkout, form validation, order confirmation
5. Cross-browser: chromium, firefox, webkit
6. Mobile: Pixel 7 emulation

Framework requirements:
- Full POM with BasePage
- Custom fixtures for auth state
- Test data from JSON and Faker
- API setup where possible
- Tags: @smoke, @regression, @critical
- HTML + Allure reporting with test.step()
- Custom reporter writing JSON summary
- GitHub Actions pipeline with npm ci
- npm audit in pipeline
- README with setup instructions

Guide me day by day (7 days). Each day: build one feature, review, evaluate.
Day 7: full code review scored out of 10.
Score >= 8: PROJECT 1 CERTIFIED ✅
```

---

### Project 2 (Days 77–83) — API Testing Framework

#### 🧠 Master Prompt — Project 2:
```
I am learning Playwright + TypeScript. Starting Real World Project 2.

Project: API Testing Framework using Playwright's APIRequestContext
Target API: https://restful-booker.herokuapp.com

Build covering:
1. Authentication: get token, use in subsequent calls
2. CRUD operations: Create, Read, Update, Delete booking
3. Schema validation: validate response structure against TypeScript interfaces
4. Negative testing: 400, 401, 404, 500 responses
5. Data-driven tests: same endpoint with multiple data sets

Framework:
- APIClient class wrapping Playwright's request
- Response models as TypeScript interfaces
- Test data factories
- Environment-based base URLs
- Allure reporting with API request/response attached as evidence
- GitHub Actions pipeline

Guide me day by day (7 days).
End with: API TESTING CERTIFIED ✅
```

---

### Project 3 (Days 84–93) — Full Stack Framework (Architect Level)

#### 🧠 Master Prompt — Project 3:
```
I am learning Playwright + TypeScript. Starting Real World Project 3 — Architect Level.

Project: Enterprise-grade Framework for https://opensource-demo.orangehrmlive.com

This project must demonstrate ARCHITECT-LEVEL thinking:

1. Multi-layer architecture:
   - UI layer (Playwright POM)
   - API layer (REST calls for setup/teardown)
   - Data layer (test data management)
   - Reporting layer (multi-reporter)

2. Advanced design patterns:
   - Builder pattern for test data
   - Strategy pattern for cross-browser
   - Custom Reporter for Slack integration

3. Features to automate:
   - Login (admin + employee roles)
   - Employee management (CRUD via UI + API)
   - Leave management
   - Reporting module

4. DevOps:
   - Docker + docker-compose
   - GitHub Actions matrix strategy (parallel browsers + sharding)
   - Report merging from sharded runs
   - GitHub Pages hosting
   - Slack notifications
   - npm audit in pipeline
   - Dependabot for dependency updates

5. Documentation:
   - Full README
   - Architecture Decision Records (ADR)
   - Onboarding guide for new team members

Guide me over 10 days. Each day covers one architectural concern.
Final: ARCHITECT CERTIFICATION ✅
```

---

# 📦 PHASE 10 — ARCHITECT LEVEL THINKING
## 🗓️ Weeks 29–31

---

### Day 94 — Test Strategy & Architecture Decisions

#### 🧠 Master Prompt — Day 94:
```
I am learning Playwright + TypeScript. Today is Day 94 — Architect Level.

Please teach me how to THINK like a test architect:
1. Testing Pyramid vs Testing Trophy — which to use when
2. How to decide what to automate and what NOT to automate
3. Risk-based testing: prioritize by business impact
4. Architecture decisions: mono-repo vs multi-repo, shared component libraries, framework versioning
5. Stakeholder communication: presenting test strategy to non-technical leaders
6. ROI of automation: how to calculate and communicate value
7. Reporting strategy for an organisation: which reporter for which audience
   (HTML for developers, Allure for QA leads, Extent for management dashboards)
8. Dependency governance: how to manage Playwright version upgrades across multiple teams

Deliverable: write a 1-page Test Strategy document for a hypothetical new project.
Evaluate my strategy — provide architect-level feedback.
```

---

### Day 95 — Mentoring & Code Review Skills

#### 🧠 Master Prompt — Day 95:
```
I am learning Playwright + TypeScript. Today is Day 95.

Please teach me mentoring and code review skills:
1. How to conduct a good code review for automation code
2. Common mistakes junior automation engineers make
3. How to onboard a new team member to your framework
4. Writing good documentation
5. How to run a knowledge-sharing session

Exercise: I will review a sample "bad" Playwright test you provide.
You evaluate the quality of my review.
PASS: my review catches all major issues and gives constructive, specific feedback.
```

---

### Day 96 — Final Architect Certification

#### 🧠 Master Prompt — Day 96:
```
I am learning Playwright + TypeScript. Today is Day 96 — FINAL CERTIFICATION.

Please conduct my Playwright + TypeScript Architect Certification:

Round 1 — Theory (15 questions): all phases including reporting, dependency management, mobile
Round 2 — Code review (3 code snippets to review and fix)
Round 3 — Architecture challenge:
   "A fintech startup wants to build test automation from scratch.
   3 developers, 1 QA, deploy 3 times a week.
   Design their entire strategy: what reporters, what CI setup, how to manage Playwright versions, what tagging strategy, what folder structure."
   Present as you would to their CTO.

Round 4 — Live problem solving:
   Given a broken Playwright test with 5 bugs, find and fix all of them.
   Include at least 1 dependency/version issue, 1 reporting misconfiguration, 1 flaky test pattern.

Scoring:
- 85-100%: PLAYWRIGHT + TYPESCRIPT ARCHITECT CERTIFIED 🏆
- 70-84%: Senior Practitioner ✅ — 2 weeks more practice recommended
- Below 70%: targeted revision plan

Congratulations on completing this journey. 🎭
```

---

# 📦 PHASE 11 — INTERVIEW READINESS
## 🗓️ Week 32
*Previously missing — critical for career transition*

---

### Day 97 — Common Interview Questions & Answers

#### 🧠 Master Prompt — Day 97:
```
I am learning Playwright + TypeScript. Today is Day 97 — Interview Preparation.

Please teach me the most commonly asked Playwright + TypeScript interview questions and how to answer them confidently:

CATEGORY 1 — Playwright fundamentals:
- What is Playwright and how does it differ from Selenium?
- Explain Playwright's auto-waiting mechanism
- What is a browser context and why is it useful?
- How do you handle iframes in Playwright?
- What is the Trace Viewer and when would you use it?

CATEGORY 2 — Framework design:
- Explain your Page Object Model implementation
- How do Playwright fixtures differ from beforeEach/afterEach?
- How do you manage test data in your framework?
- How would you handle parallel test execution?

CATEGORY 3 — Reporting:
- What reporters have you used and when would you choose Allure over built-in HTML?
- How do you attach evidence (screenshots, API logs) to test reports?
- How do you merge reports from sharded test runs?

CATEGORY 4 — CI/CD and dependencies:
- What is the difference between npm install and npm ci?
- What is semantic versioning and how does it affect your test framework?
- How do you manage Playwright browser upgrades in a team?

CATEGORY 5 — Advanced:
- How do you debug a flaky test?
- How would you build a custom reporter?
- What is the difference between page.evaluate() and page.exposeFunction()?

For each question: teach me the ideal answer structure (STAR format where applicable) and what interviewers are really testing.
Then quiz me — ask me 5 random questions and evaluate my answers.
PASS: answers demonstrate real understanding, not memorized definitions.
```

---

### Day 98 — Mock Interview Simulation

#### 🧠 Master Prompt — Day 98:
```
I am learning Playwright + TypeScript. Today is Day 98 — Mock Interview.

Please conduct a realistic 45-minute mock technical interview for a Senior Automation Engineer / SDET role requiring Playwright + TypeScript.

Structure:
- 5 min: introduction questions (tell me about your Selenium background, why Playwright)
- 15 min: technical questions (mix of theory and code review)
- 15 min: live coding challenge (write a Page Object and test for a given scenario)
- 10 min: scenario questions ("how would you handle X in your framework")

Be a strict but fair interviewer. Do not give hints.
After the interview: provide detailed feedback:
- What I answered well
- Where I showed gaps
- What a strong answer would have looked like
- Overall interview rating: [NOT READY / BORDERLINE / HIRE / STRONG HIRE]
```

---

### Day 99 — Portfolio Building & Career Readiness

#### 🧠 Master Prompt — Day 99:
```
I am learning Playwright + TypeScript. Today is Day 99 — Portfolio and Career.

Please guide me on:
1. GitHub profile optimisation for a QA automation engineer:
   - Profile README template
   - Pinned repositories
   - How to present test automation projects attractively
2. README template for a Playwright framework project:
   - What sections to include (overview, tech stack, architecture, setup, running tests, reports)
   - How to add badges (build status, coverage, etc.)
3. LinkedIn profile updates:
   - How to describe your Playwright skills
   - Keywords recruiters and ATS systems look for
4. Resume bullet points:
   - How to describe your automation projects quantitatively (reduced test time by X%, increased coverage by Y%)
5. Take-home challenge patterns:
   - What companies typically ask (basic test, POM, API test, CI setup)
   - How to time-box your response (2-3 hours max)
   - What impresses reviewers beyond working code (structure, README, comments)
6. Staying current:
   - Playwright GitHub releases — how to track breaking changes
   - Playwright Discord community
   - How to contribute to open source Playwright ecosystem

Deliverable: write your GitHub project README for your saucedemo framework.
I will review it and give feedback on clarity, completeness, and professionalism.
```

---

### Day 100 — Journey Complete 🏆

#### 🧠 Master Prompt — Day 100:
```
I am on Day 100 of my Playwright + TypeScript mastery journey.

Please help me:
1. Reflect on the entire journey — what I have built and learned
2. Create a personal skills inventory: list every tool, concept, and pattern I now know
3. Identify my top 3 strongest areas and my 2 remaining growth areas
4. Build a 90-day post-roadmap plan: what to keep practicing, what to learn next
   (Component testing, Playwright MCP, performance testing, BrowserStack/Sauce integration)
5. Write a LinkedIn post announcing my Playwright certification journey

Final message: You started as a Java Selenium engineer who was intimidated by npm.
Today you can design, build, and operate enterprise-grade Playwright frameworks.
You can debug flaky tests, merge sharded reports, build custom reporters, manage dependencies, and mentor others.
That is not a beginner. That is a practitioner. 🎭
```

---

# 📊 COMPLETE PROGRESS TRACKER

```
PLAYWRIGHT MASTERY PROGRESS TRACKER v2.0
==========================================
Phase 0  — Mindset & Setup:              [ ] Days 1–7
Phase 1  — JavaScript Essentials:        [ ] Days 8–20
Phase 2  — TypeScript Fundamentals:      [ ] Days 21–26
Phase 3  — Node.js, npm & Deps:          [ ] Days 27–31
Phase 4  — Playwright Core:              [ ] Days 32–43
Phase 5  — Reporting Mastery:            [ ] Days 44–50
Phase 6  — Framework Design & POM:       [ ] Days 51–58
Phase 7  — Advanced Playwright:          [ ] Days 59–64
Phase 8  — CI/CD & DevOps:               [ ] Days 65–69
Phase 9  — Real World Projects:          [ ] Days 70–93
Phase 10 — Architect Level:              [ ] Days 94–96
Phase 11 — Interview Readiness:          [ ] Days 97–100

Current Day: ___
Current Concept: ___
Last Score: ___/___
Struggling With: ___
Sessions Completed: ___
```

---

# 🚨 GOLDEN RULES (Never Break These)

1. Never skip a day's concept — every day is a building block
2. Never just read — always code — passive learning is fake learning
3. Never ignore a PASS/FAIL — the evaluation exists for a reason
4. Never use `any` in TypeScript — it defeats the purpose
5. Never use `waitForTimeout` in production tests — it is a code smell
6. Never put raw locators in test files — that is what Page Objects are for
7. Never commit secrets or .env files to Git
8. Never skip revision days — your brain needs consolidation
9. Always use `npm ci` in CI pipelines, never `npm install`
10. Always run `npx playwright install` after upgrading @playwright/test version

---

# 🎯 WHAT YOU WILL BE ABLE TO DO AT THE END

- Design and build enterprise-grade Playwright frameworks from scratch
- Master every reporting tool: built-in, Allure, Extent, custom reporters, ReportPortal
- Read and analyse Playwright traces to diagnose any failure
- Manage dependencies professionally: semver, auditing, safe upgrades, monorepos
- Build and run mobile/device emulation test suites
- Debug flaky tests systematically using Inspector, traces, and CDP
- Lead automation strategy for any organisation
- Integrate automation into any CI/CD pipeline with Docker and sharding
- Mentor junior engineers, conduct code reviews, present to stakeholders
- Walk into any Playwright interview fully prepared 🏆
