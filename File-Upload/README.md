# File Upload in Rust (Rocket)

This directory contains a learning project focused on handling file uploads in Rust using the Rocket web framework and `rocket_multipart_form_data` crate.

## How it Works

1. **Endpoint**: We have a POST endpoint `/upload` that accepts `multipart/form-data`.
2. **File Size Limit**: We configure restrictions on the incoming payload restricting files to a maximum size of 32MB by targeting the field named `file`.
3. **Parsing**: When an upload request is received, `MultipartFormData::parse` scans the multipart body, segregating the data. It automatically saves uploaded files as temporary files on the OS disk.
4. **Extraction**: The parsed data allows us to extract metadata like the original `file_name`, the `content_type`, and the `path` to the temporary file on disk.

### Saving the File (The Buffer Way)

The logic for taking the temporary file and saving it locally involves these steps:

- `File::create(filename)`: Creates a new, permanent file in the current working directory using the original uploaded file's name.
- `let path = &file_field.path`: Gets the path to the temporary file on disk created by the multipart parser.
- `File::open(path)`: Opens the temporary file for reading.
- `Vec::new()`: Creates a new in-memory buffer (RAM) to hold the file contents.
- `temp_file.read_to_end(&mut buffer)`: Reads everything from the temporary file into the RAM buffer.
- `file.write_all(&buffer)`: Writes the buffer's data into the new permanent file.

## Quick Tip: A More Memory-Efficient Approach

While the manual buffer method (`Vec::new()` + `read_to_end`) works perfectly to understand the flow of data, it can be dangerous for large files. Since the file limit is 32MB, reading the entire file into memory means every single upload consumes up to 32MB of RAM. If multiple requests happen at once, your server could run out of memory.

Instead of writing a manual buffer, you can let the OS handle the file copy directly without loading everything into memory. You can replace the entire buffer sequence with a single line using `tokio::fs::copy`:

```rust
// Instead of creating a buffer, reading, and writing...
// Do it in one efficient OS-level copy operation:
tokio::fs::copy(&file_field.path, filename.clone().unwrap()).await.unwrap();
```
