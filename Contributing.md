# Contributing Guidelines

Thank you for your interest in contributing to **FerrumChat** during **Nexus Spring of Code**!  
We welcome beginners and experienced developers alike. Adhering to these guidelines is **mandatory** to receive points.

---

## 🚨 CRITICAL RULES 
2.  **Get Roles**: Take the **Mentee** role in `#self-roles` and the **Project Role** in `#project-roles`.
3.  **Issue Assignment**: Issues will only be assigned after **April 16th**. Do NOT start working until assigned.
4.  **Tags are Points**:
    *   **Easy**: 3 Points
    *   **Medium**: 7 Points
    *   **Hard**: 10 Points
    

---

## 🛠️ Step-by-Step Contribution Workflow

### 1. Find or Create an Issue
*   **Creating an Issue**: You MUST use the following format or it will be closed.
    *   **Required Tag**: `nsoc`
    *   **One Difficulty Tag**: `easy`, `medium`, or `hard` (Zero other difficulty tags).
    *   **Format**:
        *   Clear description
        *   Screenshots (if UI/Frontend)
        *   Steps to reproduce (if Bug)

### 2. Wait for Assignment
*   A Project Admin or Mentor must assign the issue to you.
*   **Do not create a PR before assignment.**

### 3. Fork & Clone
```bash
git clone https://(https://github.com/SATVIKsynopsis/FerrumChat).git
cd FerrumChat
```

### 4. Create a Branch
```bash
git checkout -b your-feature-name
```

### 5. Make Changes & Commit
* Write clean, readable code.
* cargo fmt
* cargo clippy
* cargo build
* println!("debug");
* Remove all debug macros like dbg!() and unnecessary prints.
* Ensure code passes cargo build and cargo test before PR submission.

### 6. Submit a Pull Request (PR)
*   **Title**: Clear and descriptive (e.g., "Fix: Navigation overlap on mobile").
*   **Description**:
    *   Mention the issue: `Closes #IssueNumber`
    *   Explain your changes.
    *   **Screenshots**: MANDATORY for UI changes.
*   **Tags**:
    *   request the **`nsoc`** tag.
    *   request the **Difficulty Tag** (`easy`/`medium`/`hard`) that matches your issue.

---

## ⚠️ Important Notes
*   **PRs without `nsoc` and a difficulty tag will NOT receive points.**
*   Mentees must request mentors to add these tags if they are missing.
*   Violating rules or offensive behavior results in disqualification.

Happy Coding! 🚀
