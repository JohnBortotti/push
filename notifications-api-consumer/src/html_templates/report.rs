use super::super::Message;

pub fn build(message: Message) -> String {
    format!(
        r#"<!DOCTYPE html>
<html>

<head>
    <meta http-equiv="Content-Type" content="text/html; charset=UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
    <style type="text/css">
        #outlook a {{
            padding: 0;
        }}

        body {{
            margin: 0;
            padding: 0;
            -webkit-text-size-adjust: 100%;
            -ms-text-size-adjust: 100%;
        }}

        table,
        td {{
            border-collapse: collapse;
            mso-table-lspace: 0pt;
            mso-table-rspace: 0pt;
        }}

        img {{
            border: 0;
            height: auto;
            line-height: 100%;
            outline: none;
            text-decoration: none;
            -ms-interpolation-mode: bicubic;
        }}

        p {{
            display: block;
            margin: 13px 0;
        }}
    </style>

<body style="background-color:#ffffff;">
    <div style="background-color:#ffffff;">

        <div style="margin:0px auto;max-width:600px;">
            <table align="center" border="0" cellpadding="0" cellspacing="0" role="presentation" style="width:100%;">
                <tbody>
                    <tr>
                        <td style="direction:ltr;font-size:0px;padding:20px 0;padding-bottom:0px;text-align:center;">
                            <div class="mj-column-per-100 mj-outlook-group-fix"
                                style="font-size:0px;text-align:left;direction:ltr;display:inline-block;vertical-align:top;width:100%;">
                                <table border="0" cellpadding="0" cellspacing="0" role="presentation"
                                    style="vertical-align:top;" width="100%">
                                    <tbody>
                                        <tr>
                                            <td align="left"
                                                style="font-size:0px;padding:10px 25px;word-break:break-word;">
                                                <div
                                                    style="font-family:Helvetica, Arial, sans-serif;font-size:18px;font-weight:400;line-height:24px;text-align:left;color:#434245;">
                                                    <h1
                                                        style="margin: 0; font-size: 24px; line-height: normal; font-weight: bold;">
                                                        Homelab Notification </h1>
                                                </div>
                                            </td>
                                        </tr>
                                    </tbody>
                                </table>
                            </div>

                        </td>
                    </tr>
                </tbody>
            </table>
        </div>

        <div style="margin:0px auto;max-width:600px;">
            <table align="center" border="0" cellpadding="0" cellspacing="0" role="presentation" style="width:100%;">
                <tbody>
                    <tr>
                        <td style="direction:ltr;font-size:0px;padding:0;text-align:center;">

                            <div class="mj-column-per-100 mj-outlook-group-fix"
                                style="font-size:0px;text-align:left;direction:ltr;display:inline-block;vertical-align:top;width:100%;">
                                <table border="0" cellpadding="0" cellspacing="0" role="presentation"
                                    style="vertical-align:top;" width="100%">
                                    <tbody>
                                        <tr>
                                            <td style="font-size:0px;word-break:break-word;">
                                                <div style="height:20px;">   </div>
                                            </td>
                                        </tr>
                                    </tbody>
                                </table>
                            </div>

                        </td>
                    </tr>
                </tbody>
            </table>
        </div>

        <div style="background:#FFCDB0;background-color:#FFCDB0;margin:0px auto;border-radius:4px;max-width:600px;">
            <table align="center" border="0" cellpadding="0" cellspacing="0" role="presentation" style="background:#BFFCFD;background-color:#BFFCFD;width:100%;border-radius:4px;">
                <tbody>
                    <tr>
                        <td style="direction:ltr;font-size:0px;padding:20px 0;text-align:center;">
                            <div class="mj-column-per-100 mj-outlook-group-fix"
                                style="font-size:0px;text-align:left;direction:ltr;display:inline-block;vertical-align:top;width:100%;">
                                <table border="0" cellpadding="0" cellspacing="0" role="presentation"
                                    style="vertical-align:top;" width="100%">
                                    <tbody>
                                        <tr>
                                            <td align="left"
                                                style="font-size:0px;padding:10px 25px;word-break:break-word;">
                                                <div
                                                    style="font-family:Helvetica, Arial, sans-serif;font-size:18px;font-weight:bold;line-height:24px;text-align:left;color:#053878;">
                                                    <p class="date"
                                                        style="margin: 0; margin-bottom: 5px; font-size: 16px;">REPORT
                                                    </p>
                                                    <h2
                                                        style="margin: 0; font-size: 24px; font-weight: bold; line-height: 24px;">
                                                        {}</h2>
                                                </div>H
                                            </td>
                                        </tr>
                                        <tr>
                                            <td align="left"
                                                style="font-size:0px;padding:10px 25px;word-break:break-word;">
                                                <div
                                                    style="font-family:Helvetica, Arial, sans-serif;font-size:18px;font-weight:400;line-height:24px;text-align:left;color:#053878;">
                                                    <p style="margin: 0;">{}</p>
                                                    <br>
                                                    <p style="margin: 0;"><b>Message created at:</b> {} </p>
                                                </div>
                                            </td>
                                        </tr>
                                    </tbody>
                                </table>
                            </div>

                        </td>
                    </tr>
                </tbody>
            </table>
        </div>

    </div>

</body>

</html>"#,
        message.title, message.description, message.timestamp
    )
}
