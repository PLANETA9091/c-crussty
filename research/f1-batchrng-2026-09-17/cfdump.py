#!/usr/bin/env python3
"""Minimal classfile parser/disassembler for F1 STEP-0 (javap-equivalent anatomy).

Reads a .class file, prints: access flags, fields (with flags), methods with
Code attribute disassembly (common opcodes), and LDC constants resolved.
"""
import struct, sys

OPCODES = {
0x00:'nop',0x01:'aconst_null',0x02:'iconst_m1',0x03:'iconst_0',0x04:'iconst_1',0x05:'iconst_2',
0x06:'iconst_3',0x07:'iconst_4',0x08:'iconst_5',0x09:'lconst_0',0x0a:'lconst_1',0x0b:'fconst_0',
0x0c:'fconst_1',0x0d:'fconst_2',0x0e:'dconst_0',0x0f:'dconst_1',0x10:'bipush:2',0x11:'sipush:3',
0x12:'ldc:2',0x13:'ldc_w:3',0x14:'ldc2_w:3',0x15:'iload:2',0x16:'lload:2',0x17:'fload:2',0x18:'dload:2',
0x19:'aload:2',0x1a:'iload_0',0x1b:'iload_1',0x1c:'iload_2',0x1d:'iload_3',0x1e:'lload_0',0x1f:'lload_1',
0x20:'lload_2',0x21:'lload_3',0x22:'fload_0',0x23:'fload_1',0x24:'fload_2',0x25:'fload_3',0x26:'dload_0',
0x27:'dload_1',0x28:'dload_2',0x29:'dload_3',0x2a:'aload_0',0x2b:'aload_1',0x2c:'aload_2',0x2d:'aload_3',
0x2e:'iaload',0x2f:'laload',0x30:'faload',0x31:'daload',0x32:'aaload',0x33:'baload',0x34:'caload',0x35:'saload',
0x36:'istore:2',0x37:'lstore:2',0x38:'fstore:2',0x39:'dstore:2',0x3a:'astore:2',0x3b:'istore_0',0x3c:'istore_1',
0x3d:'istore_2',0x3e:'istore_3',0x3f:'lstore_0',0x40:'lstore_1',0x41:'lstore_2',0x42:'lstore_3',0x43:'fstore_0',
0x44:'fstore_1',0x45:'fstore_2',0x46:'fstore_3',0x47:'dstore_0',0x48:'dstore_1',0x49:'dstore_2',0x4a:'dstore_3',
0x4b:'astore_0',0x4c:'astore_1',0x4d:'astore_2',0x4e:'astore_3',0x4f:'iastore',0x50:'lastore',0x51:'fastore',
0x52:'dastore',0x53:'aastore',0x54:'bastore',0x55:'castore',0x56:'sastore',0x57:'pop',0x58:'pop2',0x59:'dup',
0x5a:'dup_x1',0x5b:'dup_x2',0x5c:'dup2',0x5d:'dup2_x1',0x5e:'dup2_x2',0x5f:'swap',0x60:'iadd',0x61:'ladd',
0x62:'fadd',0x63:'dadd',0x64:'isub',0x65:'lsub',0x66:'fsub',0x67:'dsub',0x68:'imul',0x69:'lmul',0x6a:'fmul',
0x6b:'dmul',0x6c:'idiv',0x6d:'ldiv',0x6e:'fdiv',0x6f:'ddiv',0x70:'irem',0x71:'lrem',0x72:'frem',0x73:'drem',
0x74:'ineg',0x75:'lneg',0x76:'fneg',0x77:'dneg',0x78:'ishl',0x79:'lshl',0x7a:'ishr',0x7b:'lshr',0x7c:'iushr',
0x7d:'lushr',0x7e:'iand',0x7f:'land',0x80:'ior',0x81:'lor',0x82:'ixor',0x83:'lxor',0x84:'iinc:3',
0x85:'i2l',0x86:'i2f',0x87:'i2d',0x88:'l2i',0x89:'l2f',0x8a:'l2d',0x8b:'f2i',0x8c:'f2l',0x8d:'f2d',
0x8e:'d2i',0x8f:'d2l',0x90:'d2f',0x91:'i2b',0x92:'i2c',0x93:'i2s',0x94:'lcmp',0x95:'fcmpl',0x96:'fcmpg',
0x97:'dcmpl',0x98:'dcmpg',0x99:'ifeq:3',0x9a:'ifne:3',0x9b:'iflt:3',0x9c:'ifge:3',0x9d:'ifgt:3',0x9e:'ifle:3',
0x9f:'if_icmpeq:3',0xa0:'if_icmpne:3',0xa1:'if_icmplt:3',0xa2:'if_icmpge:3',0xa3:'if_icmpgt:3',0xa4:'if_icmple:3',
0xa5:'if_acmpeq:3',0xa6:'if_acmpne:3',0xa7:'goto:3',0xa8:'jsr:3',0xa9:'ret:2',0xaa:'tableswitch',
0xab:'lookupswitch',0xac:'ireturn',0xad:'lreturn',0xae:'freturn',0xaf:'dreturn',0xb0:'areturn',0xb1:'return',
0xb2:'getstatic:3',0xb3:'putstatic:3',0xb4:'getfield:3',0xb5:'putfield:3',0xb6:'invokevirtual:3',
0xb7:'invokespecial:3',0xb8:'invokestatic:3',0xb9:'invokeinterface:5',0xba:'invokedynamic:5',
0xbb:'new:3',0xbc:'newarray:2',0xbd:'anewarray:3',0xbe:'arraylength',0xbf:'athrow',0xc0:'checkcast:3',
0xc1:'instanceof:3',0xc2:'monitorenter',0xc3:'monitorexit',0xc4:'wide',0xc5:'multianewarray:4',
0xc6:'ifnull:3',0xc7:'ifnonnull:3',0xc8:'goto_w:5',0xc9:'jsr_w:5',
}

class R:
    def __init__(self, b): self.b=b; self.o=0
    def u1(self): v=self.b[self.o]; self.o+=1; return v
    def u2(self): v=struct.unpack_from('>H',self.b,self.o)[0]; self.o+=2; return v
    def u4(self): v=struct.unpack_from('>I',self.b,self.o)[0]; self.o+=4; return v
    def raw(self,n): v=self.b[self.o:self.o+n]; self.o+=n; return v

def parse(path):
    b=open(path,'rb').read()
    r=R(b)
    assert r.u4()==0xCAFEBABE, "not a class file"
    minor,major=r.u2(),r.u2()
    n=r.u2()
    cp=[None]*n
    i=1
    while i<n:
        tag=r.u1()
        if tag==1: ln=r.u2(); cp[i]=('Utf8', r.raw(ln).decode('utf-8','replace'))
        elif tag==3: cp[i]=('Int', struct.unpack('>i',r.raw(4))[0])
        elif tag==4: cp[i]=('Float', r.raw(4))
        elif tag==5: cp[i]=('Long', struct.unpack('>q',r.raw(8))[0]); i+=1
        elif tag==6: cp[i]=('Double', r.raw(8)); i+=1
        elif tag==7: cp[i]=('Class', r.u2())
        elif tag==8: cp[i]=('Str', r.u2())
        elif tag==9: cp[i]=('Fieldref', r.u2(), r.u2())
        elif tag==10: cp[i]=('Methodref', r.u2(), r.u2())
        elif tag==11: cp[i]=('IfaceMethodref', r.u2(), r.u2())
        elif tag==12: cp[i]=('NameAndType', r.u2(), r.u2())
        elif tag==15: cp[i]=('MethodHandle*', r.u1(), r.u2())
        elif tag==16: cp[i]=('MethodType', r.u2())
        elif tag==17: cp[i]=('Dynamic', r.u2(), r.u2())
        elif tag==18: cp[i]=('InvokeDynamic', r.u2(), r.u2())
        elif tag==19: cp[i]=('Module', r.u2())
        elif tag==20: cp[i]=('Package', r.u2())
        else: raise ValueError(f"tag {tag} at {i}")
        i+=1

    def utf(idx): return cp[idx][1] if cp[idx] and cp[idx][0]=='Utf8' else f'#cp{idx}'
    def cls(idx):
        if idx is None or idx>=n: return '?'
        e=cp[idx]
        if e and e[0]=='Class': return utf(e[1])
        return f'#cp{idx}'
    def nat(idx):
        e=cp[idx]
        if e and e[0] in ('Fieldref','Methodref','IfaceMethodref'):
            c=cls(e[1]); nt=cp[e[2]]
            return f'{c}.{utf(nt[1])}:{utf(nt[2])}'
        return f'#cp{idx}'
    def ldc(idx):
        e=cp[idx]
        if not e: return f'#cp{idx}'
        if e[0]=='Int': return f'int {e[1]}'
        if e[0]=='Long': return f'long {e[1]}'
        if e[0]=='Str': return f'str "{utf(e[1])}"'
        if e[0]=='Class': return f'class {cls(idx)}'
        if e[0]=='Float': return f'float {e[1]}'
        return f'{e[0]}@{idx}'

    flags=major_minor=(major,major)  # placeholder
    acc, this_c, sup = r.u2(), r.u2(), r.u2()
    print(f'class {cls(this_c)} extends {cls(sup)}  flags=0x{acc:04x}  major={major}')
    # interfaces
    for _ in range(r.u2()): r.u2()
    n_f=r.u2()
    print(f'--- fields ({n_f}) ---')
    for _ in range(n_f):
        fa, fn, fd = r.u2(), r.u2(), r.u2()
        print(f'  {"0x%04x"%fa} {utf(fn)} : {utf(fd)}')
        na=r.u2()
        for _ in range(na):
            an=r.u2(); alen=r.u4(); data=r.raw(alen)
            if utf(an)=='ConstantValue' and alen==2:
                ci=struct.unpack('>H',data)[0]
                print(f'      ConstantValue = {ldc(ci)}')
    n_m=r.u2()
    print(f'--- methods ({n_m}) ---')
    for _ in range(n_m):
        ma, mn, md = r.u2(), r.u2(), r.u2()
        print(f'  {"0x%04x"%ma} {utf(mn)}{utf(md)}')
        na=r.u2()
        for _ in range(na):
            an=r.u2(); alen=r.u4(); data=r.raw(alen)
            if utf(an)=='Code':
                cr=R(data)
                mx, mn_, clen = cr.u2(), cr.u2(), cr.u4()
                code=cr.raw(clen)
                print(f'      Code: stack={mx} locals={mn_} len={clen}')
                # skip exception table
                et=cr.u2()
                for _ in range(et): cr.u2(); cr.u2(); cr.u2(); cr.u2()
                # attributes inside code (skip)
                ca=cr.u2()
                for _ in range(ca): cr.u2(); cr.u4(); cr.raw(0) if False else None
                # disassemble
                o=0
                while o<len(code):
                    op=code[o]
                    name=OPCODES.get(op, f'0x{op:02x}?')
                    extra=''
                    parts=name.split(':')
                    base=parts[0]; sz=int(parts[1]) if len(parts)>1 else 1
                    if base in ('ldc',):
                        idx=code[o+1]
                        extra=f' {ldc(idx)}' if idx<n else f' #{idx}'
                    elif base in ('ldc_w','ldc2_w'):
                        idx=struct.unpack_from('>H',code,o+1)[0]
                        extra=f' {ldc(idx)}' if idx<n else f' #{idx}'
                    elif base in ('getstatic','putstatic','getfield','putfield','invokevirtual','invokespecial','invokestatic'):
                        idx=struct.unpack_from('>H',code,o+1)[0]
                        extra=f' {nat(idx)}' if idx<n else f' #{idx}'
                    elif base=='invokeinterface':
                        idx=struct.unpack_from('>H',code,o+1)[0]
                        extra=f' {nat(idx)}' if idx<n else f' #{idx}'
                    elif base=='invokedynamic':
                        idx=struct.unpack_from('>H',code,o+1)[0]
                        extra=f' dyn#{idx}'
                    elif base in ('new','anewarray','checkcast','instanceof'):
                        idx=struct.unpack_from('>H',code,o+1)[0]
                        extra=f' {cls(idx)}' if idx<n else f' #{idx}'
                    elif base in ('ifeq','ifne','iflt','ifge','ifgt','ifle','if_icmpeq','if_icmpne','if_icmplt',
                                  'if_icmpge','if_icmpgt','if_icmple','if_acmpeq','if_acmpne','goto','jsr',
                                  'ifnull','ifnonnull'):
                        off=struct.unpack_from('>h',code,o+1)[0]
                        extra=f' -> {o+off}'
                    elif base=='goto_w' or base=='jsr_w':
                        off=struct.unpack_from('>i',code,o+1)[0]
                        extra=f' -> {o+off}'
                    elif base=='iinc':
                        extra=f' {code[o+1]} {struct.unpack_from(">b",code,o+2)[0]}'
                    elif base=='bipush':
                        extra=f' {struct.unpack_from(">b",code,o+1)[0]}'
                    elif base=='sipush':
                        extra=f' {struct.unpack_from(">h",code,o+1)[0]}'
                    elif base in ('tableswitch','lookupswitch'):
                        pad=(4-((o+1)%4))%4
                        extra=' <switch>'
                    print(f'      {o:5d}: {base}{extra}')
                    if base=='wide':
                        sub=code[o+1]
                        base2=OPCODES.get(sub,str(sub))
                        if sub==0x84: o+=6
                        else: o+=4
                        print(f'      {o:5d}: (wide {base2})')
                        continue
                    o+=sz

if __name__=='__main__':
    parse(sys.argv[1])
