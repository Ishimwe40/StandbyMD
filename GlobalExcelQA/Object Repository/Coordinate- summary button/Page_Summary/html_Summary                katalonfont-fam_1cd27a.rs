<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>html_Summary                katalonfont-fam_1cd27a</name>
   <tag></tag>
   <elementGuidId>8c65fa1d-4abc-416f-919d-0fa24df28ce5</elementGuidId>
   <selectorCollection>
      <entry>
         <key>XPATH</key>
         <value>//*/text()[normalize-space(.)='']/parent::*</value>
      </entry>
   </selectorCollection>
   <selectorMethod>XPATH</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>html</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>
    
    Summary
    
    

    
    
#katalon{font-family:monospace;font-size:13px;background-color:rgba(0,0,0,.7);position:fixed;top:0;left:0;right:0;display:block;z-index:999999999;line-height: normal} #katalon div{padding:0;margin:0;color:#fff;} #katalon kbd{display:inline-block;padding:3px 5px;font:13px Consolas,&quot;Liberation Mono&quot;,Menlo,Courier,monospace;line-height:10px;color:#555;vertical-align:middle;background-color:#fcfcfc;border:1px solid #ccc;border-bottom-color:#bbb;border-radius:3px;box-shadow:inset 0 -1px 0 #bbb;font-weight: bold} div#katalon-spy_elementInfoDiv {color: lightblue; padding: 0px 5px 5px} div#katalon-spy_instructionDiv {padding: 5px 5px 2.5px}

    
    

    
    
    

    




    var actionPageOne = '/en-US/Report/VCardPageOne';
    var actionPageTwo = '/en-US/Report/VCardPageTwo';
    var tryAgain = 'An error occured, please try again';
    $(document).ready(function () {
        $('body').loading({
            stoppable: false
        });
        //First page
        $.ajax({
            url: actionPageOne,
            data: {
                identifier: 'SBMD202001270944',
                token: '95b2085a-a398-4efb-be08-1c5a18eac2e8'
            },
            type: 'POST',
            success: function (data) {
                $('#pageOne').html(data);

                //Second Page
                $.ajax({
                    url: actionPageTwo,
                    data: {
                        identifier: 'SBMD202001270944',
                        token: '95b2085a-a398-4efb-be08-1c5a18eac2e8'
                    },
                    type: 'POST',
                    success: function (data) {
                        $('#pageTwo').html(data);
                        debugger;
                        var $div = $('body').find('.loading-shown');
                        $div.css('display', 'none');
                        //$('body').loading('stop');

                    },
                    error: function (jqXHR, textStatus, errorThrown) {
                        showError(errorThrown);

                        HideLoader();
                    }
                });

            },
            error: function (jqXHR, textStatus, errorThrown) {
                showError(errorThrown);
                HideLoader();
            }
        });


    });


Loading.../html[1]</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>/html[1]</value>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>//*/text()[normalize-space(.)='']/parent::*</value>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//html</value>
   </webElementXpaths>
</WebElementEntity>
