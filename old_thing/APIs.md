# APIs清单
## 均为JSON格式的请求和返回数据，不能用作页面跳转
## 请求成功还是失败主要通过code的值判断，200为成功

---

使用用户信息创建token
/token/create
- 方法为POST
  - requestBody为
    - {"user_id":1, "password":"123456"}
- 返回结果
  - {"code": "200", "msg":"请求成功", "data":"tokenXXXXXX"}

通过token返回当前用户
/token/get_user
- 方法为GET
- 返回结果为
  - {"code":"200","msg":"请求成功","data":{"user_id":0,"user_name":"test_test","password":"123456","points":0,"role_id":0,"role_name":"测试","role_permissions":"用于测试"}}

测试与后端的连接
/index
- 方法为GET
- 返回结果 
  - {"code":"200","msg":"请求成功","data":"这里是主页"}

登录
/log_in
- 方法为POST
- requestBody为
  - {
    "user_name": "test_test",
    "password": "123456"
    }
- 返回结果为
  - 登录成功
    - {"code":"200","msg":"请求成功","data":{"user_id":0,"user_name":"test_test","password":"123456","points":0,"role_id":0,"role_name":"测试","role_permissions":"用于测试"}}
  - 登录失败
    - {"code":"500","msg":"密码12345不正确","data":null}

注册，仅添加用户，角色信息不会指定，需要再调用users/update来指定用户的角色
/sign_in
- 方法为POST
- requestBody为
  - {
    "user_name": "wuppo",
    "password": "1234512345"
    }
- 返回结果为
  - 注册成功
    - {"code":"200","msg":"请求成功","data":"注册成功"}
  - 注册失败
    - {"code":"500","msg":"用户wuppo已存在","data":null}


查询数据库中的所有的用户
/users
- 方法为GET
- 返回结果为
  - {"code":"200","msg":"请求成功","data":[{"user_id":0,"user_name":"test_test","password":"123456","points":0,"role_id":0,"role_name":"测试","role_permissions":"用于测试"},{"user_id":1,"user_name":"test_t","password":"123456","points":0,"role_id":1,"role_name":"审核","role_permissions":"用于审核"},{"user_id":3,"user_name":"wuppo","password":"1234512345","points":0,"role_id":0,"role_name":"测试","role_permissions":"用于测试"}]}


通过用户id来删除用户
/users/delete/{用户的id}
- 方法为GET
- 返回结果为
  - 删除成功
    - {"code":"200","msg":"请求成功","data":"删除成功"}
  - 删除失败
    - {"code":"500","msg":"用户2不存在","data":null}

更新用户信息，可以指定角色id
/users/update
- 方法为POST
- requestBody为
  - {"user_id": 3,"user_name": "wuppo","password": "123456789","points": 10,"role_id": 1 }
- 返回结果为
  - {"code":"200","msg":"请求成功","data":[{"user_id":1,"user_name":"test_t","password":"123456","points":0,"role_id":1,"role_name":"审核","role_permissions":"用于审核"},{"user_id":3,"user_name":"wuppo","password":"123456789","points":10,"role_id":0,"role_name":"测试","role_permissions":"用于测试"}]}

通过用户名来模糊查询
/users/search/{keyword}
- 方法为GET
- 返回结果为
  - {"code":"200","msg":"请求成功","data":[{"user_id":3,"user_name":"wuppo","password":"123456789","points":10,"role_id":0,"role_name":"测试","role_permissions":"用于测试"}]}

查询所有的角色信息
/role
- 方法为GET
- 返回结果为
  - {"code":"200","msg":"请求成功","data":[{"role_id":0,"role_name":"测试","role_permissions":"用于测试"},{"role_id":1,"role_name":"审核","role_permissions":"用于审核"}]}

添加角色到数据库中
/role/add
- 方法为POST
- requestBody为
  - {"role_name": "用户","role_permissions": "普通用户"}
- 返回结果为
  - 添加成功
    - {"code":"200","msg":"请求成功","data":[{"role_id":0,"role_name":"测试","role_permissions":"用于测试"},{"role_id":1,"role_name":"审核","role_permissions":"用于审核"},{"role_id":3,"role_name":"用户","role_permissions":"普通用户"}]}
  - 添加失败
    - {"code":"500","msg":"角色用户已存在","data":null}

通过角色id来删除对应角色
/role/delete/{role_id}
- 方法为GET
- 返回结果为
  - 删除失败
    - {"code":"500","msg":"删除成败","data":null}
  - 删除成功
    - {"code":"200","msg":"请求成功","data":"成功删除"}

通过角色的名字模糊查询
/role/search/{keyword}
- 方法为GET
- 返回结果为
  - {"code":"200","msg":"请求成功","data":[{"role_id":3,"role_name":"用户","role_permissions":"普通用户"}]}


更新数据库中的角色信息
/role/update
- 方法为POST
- requestBody为
  - {"role_id": 3,"role_name": "用户user","role_permissions": "普通用户"}
- 返回结果为
  - 修改成功
    - {"code":"200","msg":"请求成功","data":[{"role_id":0,"role_name":"测试","role_permissions":"用于测试"},{"role_id":1,"role_name":"审核","role_permissions":"用于审核"},{"role_id":3,"role_name":"用户user","role_permissions":"普通用户"}]}
  - 修改失败
    - {"code":"500","msg":"不存在该角色","data":null}


查询所有的栏目
/columns
- 方法为GET
- 返回结果为
  - {"code":"200","msg":"请求成功","data":[{"column_id":0,"column_description":"建设"},{"column_id":1,"column_description":"民生"}]}


添加栏目
/columns/add
- 方法为POST
- requestBody为
  - {"column_description": "经济"}
- 返回结果为
  - 添加成功
    - {"code":"200","msg":"请求成功","data":[{"column_id":0,"column_description":"建设"},{"column_id":1,"column_description":"民生"},{"column_id":3,"column_description":"经济"}]}
  - 添加失败
    - {"code":"500","msg":"栏目经济已存在","data":null}

通过id更新栏目信息
/columns/update
- 方法为POST
- requestBody为
  - {"column_id":3, "column_description": "经济"}
- 返回结果为
  - 更新成功
    - {"code":"200","msg":"请求成功","data":[{"column_id":0,"column_description":"建设"},{"column_id":1,"column_description":"民生"},{"column_id":3,"column_description":"经济"}]}
  - 更新失败
    - {"code":"500","msg":"栏目2不存在","data":null}

栏目的描述来模糊查询
/columns/search/{keyword}
- 方法为GET
- 返回结果为
  - {"code":"200","msg":"请求成功","data":[{"column_id":3,"column_description":"经济"}]}

通过id删除栏目
/columns/delete/{id}
- 方法为GET
- 返回结果为
  - 删除成功
    - {"code":"200","msg":"请求成功","data":"删除成功"}
  - 删除失败
    - {"code":"500","msg":"栏目3不存在","data":null}

查询所有的关键词
/keywords
- 方法为GET
- 返回结果为
  - {"code":"200","msg":"请求成功","data":[{"keyword_id":1,"keyword_description":"民心所系","column_id":1},{"keyword_id":2,"keyword_description":"人心相向","column_id":1},{"keyword_id":3,"keyword_description":"经济建设","column_id":0}]}

通过关键词的id删除关键词
/keywords/delete/{id}
- 方法为GET
- 返回结果为
  - 删除成功
    - {"code":"200","msg":"请求成功","data":"删除成功"}
  - 删除失败
    - {"code":"500","msg":"关键词3不存在","data":null}

通过关键词的描述来模糊查询
/keywords/search/{keyword}
- 方法为GET
- 返回结果为
  - {"code":"200","msg":"请求成功","data":[{"keyword_id":1,"keyword_description":"民心所系","column_id":1},{"keyword_id":2,"keyword_description":"人心相向","column_id":1}]}

列出栏目id对应的所有关键词
/keywords/in_column/{id}
- 方法为GET
- 返回结果为
  - id存在
    - {"code":"200","msg":"请求成功","data":[{"keyword_id":1,"keyword_description":"民心所系","column_id":1},{"keyword_id":2,"keyword_description":"人心相向","column_id":1}]}
  - id不存在
    - {"code":"500","msg":"栏目12不存在","data":null}

  
添加一个栏目下属的关键词
/keywords/add
- 方法为POST
- requestBody为
  - {"keyword_description":"万众一心","column_id":1}
- 返回结果为
  - 添加成功
    - {"code":"200","msg":"请求成功","data":[{"keyword_id":1,"keyword_description":"民心所系","column_id":1},{"keyword_id":2,"keyword_description":"人心相向","column_id":1},{"keyword_id":3,"keyword_description":"万众一心","column_id":1}]}
  - 添加失败
    - {"code":"500","msg":"关键词万众一心已存在","data":null}

通过关键词id更新关键词
/keywords/update
- 方法为POST
- requestBody为
  - {"keyword_id":1,"keyword_description":"军民一家亲","column_id":1}
- 返回结果为
  - 更新成功
    - {"code":"200","msg":"请求成功","data":[{"keyword_id":2,"keyword_description":"人心相向","column_id":1},{"keyword_id":3,"keyword_description":"万众一心","column_id":1},{"keyword_id":1,"keyword_description":"军民一家亲","column_id":1}]}
  - 更新失败
    - {"code":"500","msg":"栏目2不存在","data":null}

列出所有的判断题
/questions/tf
- 方法为GET
- 返回结果为
  - {"code":"200","msg":"请求成功","data":[{"true_false_question_id":1,"question_description":"这是一个连接了数据库的项目","correct_answer":"正确","question_analysis":"显然可知"}]}

通过判断题的描述信息来模糊查询
/questions/tf/search/des/{keyword}
- 方法为GET
- 返回结果为
  - {"code":"200","msg":"请求成功","data":[{"true_false_question_id":1,"question_description":"这是一个连接了数据库的项目","correct_answer":"正确","question_analysis":"显然可知"}]}

通过判断题的解析来模糊查询
/questions/tf/search/anal/{keyword}
- 方法为GET
- 返回结果为
  - {"code":"200","msg":"请求成功","data":[{"true_false_question_id":1,"question_description":"这是一个连接了数据库的项目","correct_answer":"正确","question_analysis":"显然可知"}]}

通过判断题绑定的关键词的信息来模糊查询
/questions/tf/search/keyword/{keyword}
- 方法为GET
- 返回结果为
  - {"code":"200","msg":"请求成功","data":[{"true_false_question_id":1,"question_description":"这是一个连接了数据库的项目","correct_answer":"正确","question_analysis":"显然可知"}]}


通过id来删除判断题
/questions/tf/delete/{id}
- 方法为GET
- 返回结果为
  - 删除成功
    - {"code":"200","msg":"删除成功","data":null}
  - 删除失败
    - {"code":"500","msg":"题目2不存在","data":null}

将关键词绑定到对应题目
/questions/tf/add/keyword
- 方法为POST
- requestBody为
  - {"keyword_id":1, "true_false_question_id":1}

把判断题的关键词去绑定，不会删除关键词
/questions/tf/delete/keyword
- 方法为POST
- requestBody为
  - {"keyword_id":1, "true_false_question_id":1}
- 返回结果为
  - 删除成功
  - 删除失败
    - {"code":"500","msg":"删除失败","data":null}

添加判断题
/questions/tf/add
- 方法为POST
- requestBody为
  - {"question_description": "1-1=0","correct_answer": "正确","question_analysis": "毋庸置疑"}
- 返回结果为
  - {"code":"200","msg":"请求成功","data":[{"true_false_question_id":1,"question_description":"这是一个连接了数据库的项目","correct_answer":"正确","question_analysis":"显然可知"},{"true_false_question_id":2,"question_description":"1-1=0","correct_answer":"正确","question_analysis":"毋庸置疑"}]}

通过id来更新判断题信息
/questions/tf/update
- 方法为POST
- requestBody为
  - {"true_false_question_id":1,"question_description":"1-1=1","correct_answer":"错误","question_analysis":"毋庸置疑"}
- 返回结果为
  - 更新成功
    - {"code":"200","msg":"请求成功","data":[{"true_false_question_id":1,"question_description":"1-1=1","correct_answer":"错误","question_analysis":"毋庸置疑"},{"true_false_question_id":2,"question_description":"1-1=0","correct_answer":"正确","question_analysis":"毋庸置疑"}]}
  - 更新失败
    - {"code":"500","msg":"更新失败","data":null}

