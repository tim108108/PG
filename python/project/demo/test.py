class Solution(object):
    def isValid(self, s):
        tmp = []
        for i in s:
            if not tmp :
                tmp.append(i)
            elif (i == ")" and tmp[-1] == "(") or (i == "]" and tmp[-1] == "[") or (i == "}" and tmp[-1] == "{"):
                tmp.pop()
            else:
                tmp.append(i)
        print(tmp)
        print(s)
        if not tmp:
            return True
        else :
            return False
#s="("
s="()"
#s = "()[]"
print(Solution().isValid(s))