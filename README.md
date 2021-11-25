# getmeta

### Install

Amazon:

```
sudo yum install file-devel
```

Ubuntu:

```
sudo apt-get install libmagic1
```

Python:

```
pip install getmeta
```

### Data

**1. path**
  - absolute
    - directory
    - filename
    
**2. source**
  - DIR or FILE
  
**3. size**
  - file size 
    - bytes (B)

**4. md5**
  - hash value
    - non-empty files
    - less than 100 MB

**5. sha256**
  - hash value
    - non-empty files
    - less than 100 MB

**6. magic**
  - mime type
    - non-empty files
    - less than 100 MB

**7. uid**
  - display name
    - /etc/passwd
    
**8. gid**
  - display name
    - /etc/group
    
**9. mask**
  - inode mask
    - octet

**10. mtime**
  - last modified timestamp
    - unix epoch

**11. md5path**
  - fullpath
    - md5

**12. md5dir**
  - directory
    - md5

**13. md5name**
  - filename
    - md5

**14. sha256path**
  - fullpath
    - sha256

**15. sha256dir**
  - directory
    - sha256

**16. sha256name**
  - filename
    - sha256

### Inode Mask

```
mask
0o100644
```

0o**100**644

- 170 - bit mask
- 140 - socket
- 120 - symbolic link
- 100 - regular file
- 060 - block device
- 040 - directory
- 020 - character device
- 010 - FIFO
- 004 - set-user-id bit
- 002 - set-group-id bit
- 001 - sticky bit

0o100**6**44

- 4 - user read
- 2 - user write 
- 1 - user execute 

0o1006**4**4

- 4 - group read 
- 2 - group write 
- 1 - group execute 

0o10064**4**

- 4 - world read 
- 2 - world write 
- 1 - world execute 

### References

- https://pypi.org/project/aiofile/
- https://pypi.org/project/blake3/
- https://pypi.org/project/python-magic/
- https://man7.org/linux/man-pages/man7/inode.7.html

### Local Development

```
$ python setup.py install --user
```
