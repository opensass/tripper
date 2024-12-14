<div align="center">

# 🗺️ Tripper 🤖

[![Made-with-Rust](https://img.shields.io/badge/Made%20with-Rust-1f425f.svg?logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Rust](https://img.shields.io/badge/Rust-1.79%2B-blue.svg)](https://www.rust-lang.org)
[![Maintenance](https://img.shields.io/badge/Maintained%3F-yes-green.svg)](https://github.com/wiseaidev)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

[![Open SASS Discord](https://dcbadge.limes.pink/api/server/b5JbvHW5nv)](https://discord.gg/b5JbvHW5nv)

![arch](https://github.com/user-attachments/assets/48a398bc-32fe-4416-975d-ba439a6cddbf)

</div>

## 🚀 About Tripper

Tripper is a modern travel assistant leveraging [**AWS Bedrock**](https://aws.amazon.com/bedrock/) models to enhance your trip planning and exploration. With powerful integrations, streamlined data models, and a modular design, Tripper makes organizing, customizing, and experiencing your journeys effortless.

### 🛠️ Pre-requisites

1. **Install [`rustup`](https://www.rust-lang.org/tools/install)**:

   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Install [`Dioxus CLI`](https://dioxuslabs.com/learn/0.5/getting_started)**:

   ```bash
   cargo install dioxus-cli@0.5.6
   ```

3. **Fork/Clone the Repository**:

   ```bash
   git clone https://github.com/opensass/tripper
   ```

### 🔑 Setting Up Environment Variables

Before running **Tripper**, configure the environment variables to connect to external services such as **MongoDB**, **Unsplash**, **Google Maps** and **AWS Bedrock**. Here's how:

#### Create an `.env` File

Copy the example environment file and update it with your credentials.

```bash
cp .env.example .env
```

**`.env` Variables:**

```bash
MONGODB_USR=
MONGODB_PWD=
MONGODB_CLSTR=your-cluster.mongodb.net
MONGODB_DB_NAME=tripper
JWT_SECRET=
UNSPLASH_API_KEY=
AWS_REGION=
AWS_PROFILE=
AWS_ACCESS_KEY_ID=
AWS_SECRET_ACCESS_KEY=
AWS_CONTAINER_CREDENTIALS_FULL_URI=
AWS_CONTAINER_AUTHORIZATION_TOKEN=
AWS_SDK_UA_APP_ID=
```

> [!NOTE]
> Visit the respective service portals (AWS, MongoDB, Google Maps and Unsplash) to generate any missing credentials.

### 🥑 MongoDB Setup

Follow [this guide](./MongoDB.md) to set up your MongoDB database and establish a connection with Tripper.

### 🔐 Generate a JWT Secret Key

Use OpenSSL to create a secure JWT secret key and update your `.env` file.

```bash
openssl rand -hex 128
```

### ✨ Set Up AWS Bedrock

AWS Bedrock provides the AI capabilities that power Tripper's smart recommendations and trip planning features. Ensure your **AWS Bedrock** environment is configured by setting up the required access keys and credentials in your `.env` file.

### 📸 Unsplash API

Tripper integrates with the **Unsplash API** for sourcing high-quality images. Obtain an API key from the [Unsplash Developer Portal](https://unsplash.com/oauth/applications) and include it in your `.env` file.

### 🚀 Running the Application

1. Start the client:

   ```bash
   dx serve --port 3000
   ```

2. Navigate to `http://localhost:3000` to explore the Tripper landing page.

> **Note:** The initial build might take a few minutes, but the results are worth the wait!

## ✅ Features

- Full support for AWS Bedrock models, including **Claude 3** and other advanced AI solutions.
- Intelligent trip planning with high-quality image integration.
- Secure user authentication and role management.

## 🛠️ Project Structure

- **Components**: Reusable UI components like `navbar` and `footer` ensure consistency and maintainability.
- **Server**: Organized with the **MVC** pattern for clear separation of concerns. This includes models, controllers, and response handlers.
- **Pages**: All app views (e.g., `dashboard`, `home`) are modularized for straightforward updates.

This structure keeps the project scalable and easy to navigate as it grows.

## 👨‍💻 Data Models

Tripper uses **MongoDB** for data storage, with well-defined models for efficiency and scalability:

- **User** 🧑‍💼: Manages user credentials, profiles, and roles for secure access.
- **Trip** 📚: Tracks trip details such as title, type, topics, and timestamps.
- **Detail** 📖: Stores trip daily details content in both markdown and HTML formats for flexibility.
- **Conversation** 💬: Records AI interactions for reference and analysis.
- **Message** 📝: Logs individual messages in conversations for traceability.
