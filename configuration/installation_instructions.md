# Yaak MCP Server Setup

This extension connects Zed to your Yaak instance via the Model Context Protocol (MCP).

## Prerequisites

- [Yaak](https://yaak.app/) must be running with its MCP server enabled.
- Node.js must be installed (for the `mcp-remote` bridge).

## Configuration

The extension works with the default Yaak MCP server URL (`http://127.0.0.1:64343/mcp`). If your Yaak instance uses a different URL, update the `server_url` setting in your Zed settings.