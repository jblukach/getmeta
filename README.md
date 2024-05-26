# getmeta

### NORMALIZATION

Clean data is mission-critical for collecting operating system artifacts, especially with user home directories.

### APPLE, LINUX, & UNIX

```python
    if path[:1] == '/':
        out = path.split('/')
        try:
            if out[1] == 'home':
                out[2] = 'user'
                path = '/'.join(out)
            elif out[1] == 'Users':
                if out[2] != 'Shared':
                    out[2] = 'user'
                    path = '/'.join(out)
        except:
            pass
```

### MICROSOFT WINDOWS

```python
    elif path[1] == ':':
        out = path.split('\\')
        try:
            if out[1] == 'Users' or out[1] == 'Documents and Settings':
                if out[2] != 'Default' and out[2] != 'Public' and out[2] != 'All Users' and out[2] != 'Default User':
                    out[0] = 'C:'
                    out[2] = 'Administrator'
                    path = '\\'.join(out)
        except:
            pass
```

### CAPTURED DATA

0. ami
1. path
2. file
3. size
4. md5
5. sha256
6. sha3
7. md5path
8. sha256path
9. sha3path
10. md5dir
11. sha256dir
12. sha3dir
13. md5name
14. sha256name
15. sha3name
16. type
17. entropy
18. block
19. location

### INSTALLATION

```
pip install getmeta
```

### DEVELOPMENT

```
python setup.py install --user
```

### META INFORMATION

![Meta Information](images/metainfo.png)