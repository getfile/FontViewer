
全局: 显示字体中字符的分布
	以glyph_id排序, 或以unicode排序

	unicode 的基本面(1个), 辅助面(17个), 每个面(2^16, 65536)

	一个面一个区域, 每个区域是流式布局, 每行元素数量 = 窗口宽度 / 元素宽度
	
	元素最小是2*2像素, 最大是视口宽度

	保留字符, A色
	有效字符但是无字形, B色
	有效字符有字形, C色
	背景, D色
	
	布局的宽度等于窗口宽度, 也就是没有横向滚动, 只能竖向滚动.
	左边条显示每行首元素的unicode编码

	缩到最小可一览所有字形的分布区域(大部分字体只有一部分字形)
		(字形太小也无意义, 最小应该是5像素左右, 中文最小是8像素左右)
	放大则可见字形外形

预览: 显示字符的字形
	元素无限大, 无限小(至0)
	绘制字形的样条曲线, 控制点
	有详细信息, 比如字间距,最大宽度, unicode编码等等其它
	


流程:
	字体数据 -> 解析某字符 -> 光栅化成图像字节数据 -> 生成Surface -> 更新到字符纹理 -> 字符纹理渲染到[layout纹理]
						   -> 获取外形的矢量数据 -> 渲染到[preview纹理]
	
	显示[layout纹理]和[preview纹理]

