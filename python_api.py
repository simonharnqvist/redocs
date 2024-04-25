report = HTMLReport(title, path)
report.set_style(
    font="Arial",
    bg_col="red")
report.header("Hello, world!", level=1)

report.header("Hello, section!", level=2)
report.paragraph("Put some text here", size=12, font="Verdana")
report.figure("img_path", caption="This is a figure")
report.render_html()
