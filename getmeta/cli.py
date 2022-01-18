import asyncio
import grp
import hashlib
import magic
import os
import platform
import pwd
import time
import warnings
from aiofile import async_open
from getmeta import __version__

warnings.filterwarnings('ignore')

BLOCKSIZE = 65536

run = str(int(time.time()))
host = platform.node()

async def hasher(fname):
    try:
        md5_file = ''
        sha256_file = ''
        md5_hasher = hashlib.md5()
        sha256_hasher = hashlib.sha256()
        with open(fname,'rb') as afile:
            buf = afile.read(BLOCKSIZE)
            while len(buf) > 0:
                md5_hasher.update(buf)
                sha256_hasher.update(buf)
                buf = afile.read(BLOCKSIZE)
        md5_file = md5_hasher.hexdigest().upper()
        sha256_file = sha256_hasher.hexdigest().upper()
    except:
        md5_file = '-'
        sha256_file = '-'
        pass
    if md5_file == 'D41D8CD98F00B204E9800998ECF8427E':
        md5_file = 'EMPTY'
    if sha256_file == 'E3B0C44298FC1C149AFBF4C8996FB92427AE41E4649B934CA495991B7852B855':
        sha256_file = 'EMPTY'
    hashes = md5_file+'|'+sha256_file
    return hashes

async def matchmeta(meta):
    md5_hasher = hashlib.md5()
    sha256_hasher = hashlib.sha256()
    md5_hasher.update(meta.encode())
    sha256_hasher.update(meta.encode())
    md5_meta = md5_hasher.hexdigest().upper()
    sha256_meta = sha256_hasher.hexdigest().upper()
    meta = md5_meta+'|'+sha256_meta
    return meta

async def mime(fname):
    try:
        magic_file = magic.from_file(fname, mime=True)
    except:
        magic_file = '-'
        pass
    return magic_file

async def normalizepath(path):
    if path[:1] == '/':					### LINUX
        out = path.split('/')
        try:
            if out[1] == 'home':
                out[2] = 'user'
                path = '/'.join(out)
        except:
            pass
    elif path[1] == ':': 				### WINDOWS
        new = list(path)
        new[0] = 'C'
        path = (''.join(new))
        out = path.split('\\')
        try:
            if out[1] == 'Users' or out[1] == 'Documents and Settings':
                if out[2] != 'Default' and out[2] != 'Public' and out[2] != 'All Users' and out[2] != 'Default User':
                    out[2] = 'Administrator'
                    path = '\\'.join(out)
        except:
            pass
    return path

async def parsefilename(filename):
    if filename[:1] == '/':					### LINUX
        out = filename.split('/')
        count = len(out) - 1
    elif filename[1] == ':': 				### WINDOWS
        new = list(path)
        new[0] = 'C'
        path = (''.join(new))
        out = path.split('\\')
        count = len(out) - 1
    return out[count]

async def parseonlypath(onlypath):
    if onlypath[:1] == '/':					### LINUX
        out = onlypath.split('/')
        del out[-1]
        onlypath = '/'.join(out)
    elif onlypath[1] == ':': 				### WINDOWS
        new = list(path)
        new[0] = 'C'
        path = (''.join(new))
        out = path.split('\\')
        del out[-1]
        onlypath = '\\'.join(out)
    return onlypath

async def start():
    print('--------------------------------')
    print('GETMETA v'+__version__)
    print('--------------------------------')
    async with async_open(host+'-'+run+'.txt', 'w+') as f:
        await f.write('path|source|size|md5|sha256|magic|uid|gid|mask|mtime|md5path|md5dir|md5name|sha256path|sha256dir|sha256name\n')
        for dirpath, dirs, files in os.walk('/'): # <-- ADD WINDOWS OPTION
            dname = os.path.join(dirpath)
            size = '-'
            md5_file = '-'
            sha256_file = '-'
            magic_file = '-'
            try:
                uid = pwd.getpwuid(os.stat(dname).st_uid)[0]
            except:
                uid = '-'
                pass
            try:
                gid = grp.getgrgid(os.stat(dname).st_gid)[0]
            except:
                gid = '-'
                pass
            try:
                mask = oct(os.stat(dname).st_mode)
            except:
                mask = '-'
                pass
            try:
                mtime = str(os.stat(dname).st_mtime)
            except:
                mtime = '-'
                pass
            md5_path = '-'
            sha256_path = '-'
            directory = await normalizepath(dname)
            meta = await matchmeta(directory)
            out = meta.split('|')
            md5_dir = out[0]
            sha256_dir = out[1]
            md5_name = '-'
            sha256_name = '-'
            await f.write(dname+'|DIR|'+size+'|'+md5_file+'|'+sha256_file+'|'+magic_file+'|'+ \
                          uid+'|'+gid+'|'+mask+'|'+mtime+'|'+md5_path+'|'+md5_dir+'|'+md5_name+'|'+ \
                          sha256_path+'|'+sha256_dir+'|'+sha256_name+'\n')
            for filename in files:
                fname = os.path.join(dirpath,filename)
                try:
                    size = os.path.getsize(fname)			
                except: 
                    size = 0
                    pass
                if size == 0:
                    md5_file = 'EMPTY'
                    sha256_file = 'EMPTY'
                    magic_file = 'EMPTY'
                elif size > 104857599:
                    md5_file = 'LARGE'
                    sha256_file = 'LARGE'
                    magic_file = 'LARGE'
                else:
                    hashes = await hasher(fname)
                    out = hashes.split('|')
                    md5_file = out[0]
                    sha256_file = out[1]
                    magic_file = await mime(fname)
                try:
                    uid = pwd.getpwuid(os.stat(fname).st_uid)[0]
                except:
                    uid = '-'
                    pass
                try:
                    gid = grp.getgrgid(os.stat(fname).st_gid)[0]
                except:
                    gid = '-'
                    pass
                try:
                    mask = oct(os.stat(fname).st_mode)
                except:
                    mask = '-'
                    pass
                try:
                    mtime = str(os.stat(fname).st_mtime)
                except:
                    mtime = '-'
                    pass
                fullpath = await normalizepath(fname)
                meta = await matchmeta(fullpath)
                out = meta.split('|')
                md5_path = out[0]
                sha256_path = out[1]
                directory = await parseonlypath(fullpath)
                meta = await matchmeta(directory)
                out = meta.split('|')
                md5_dir = out[0]
                sha256_dir = out[1]
                filename = await parsefilename(fullpath)
                meta = await matchmeta(filename)
                out = meta.split('|')
                md5_name = out[0]
                sha256_name = out[1]
                await f.write(fname+'|FILE|'+str(size)+'|'+md5_file+'|'+sha256_file+'|'+ \
                              magic_file+'|'+uid+'|'+gid+'|'+mask+'|'+mtime+'|'+md5_path+'|'+ \
                              md5_dir+'|'+md5_name+'|'+sha256_path+'|'+sha256_dir+'|'+sha256_name+'\n')

def main():
    asyncio.run(start())
    print('Completed!!')    
