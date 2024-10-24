<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>div_Admin Portal                           _16e6d8</name>
   <tag></tag>
   <elementGuidId>8f306245-636e-42e0-b06a-635778d9cce1</elementGuidId>
   <selectorCollection>
      <entry>
         <key>CSS</key>
         <value>div.bg-cover</value>
      </entry>
      <entry>
         <key>XPATH</key>
         <value>//div/div/div/div</value>
      </entry>
   </selectorCollection>
   <selectorMethod>XPATH</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>true</useRalativeImagePath>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>div</value>
      <webElementGuid>d12e373d-a0d1-4bbf-92ab-9bfdd3c57c68</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>class</name>
      <type>Main</type>
      <value>bg-cover</value>
      <webElementGuid>ff23dd51-86ac-4f97-962d-746f107d0663</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>
        
            
                Admin Portal
            
            
            Conduent Loan Manager User Setup and Application Configuration
            This portal provides you with access to our users and applications
        
        
            
            
                
                    Sign In
                
                
                    Sign In to manage users and configure applications.
                    
                        
                        
                        
                            
                            
                        
                        
                            User name is required.
                        
                        
                            
                            
                        
                        
                            Password is required.
                        
                        
                            
                                
                                    
                                    Remember Me
                                
                            
                            
                                Forgot your password?
                            
                        
                        Login
                    

                    
                
            
            
            
    
    

        &lt;div class=&quot;text-center text-csr-heading&quot; style=&quot;margin-top:10px;&quot; ng-show=&quot;!isSuccess&quot;>
            &lt;!--&lt;div class=&quot;empty-wrapper mt-lg&quot;>
              &lt;!--  &lt;span class=&quot;fa fa-users fa-3x&quot;>&lt;/span>&lt;br />&lt;span>CSR Portal&lt;/span>

            &lt;/div>-->
            &lt;img src=&quot;img/conduent_logo.png&quot; id=&quot;img&quot; alt=&quot;Logo&quot; class=&quot;m0&quot; style=&quot;width:200px&quot;/>
            &lt;h3 style=&quot;margin-top:5px; margin-bottom:5px;&quot;>Loan Manager&lt;/h3>
            &lt;p>Access customers loan information for maintenance and service&lt;/p>
        &lt;/div>

        &lt;div class=&quot;panel-body pb0 pt0&quot; ng-show=&quot;!isSuccess&quot;>
            &lt;div class=&quot;modal-header p0&quot;>
                &lt;h4 id=&quot;modalPasswordChange&quot; class=&quot;modal-title text-left &quot; style=&quot;font-size:16px;&quot;>Password Change Required!&lt;/h4>
            &lt;/div>
            &lt;div class=&quot;modal-body text-left pb0&quot;>Please enter your current password and your desired new password.&lt;/div>
            &lt;div class=&quot;modal-body text-left pb0 pt0&quot; style=&quot;padding-bottom:5px;&quot;>
                &lt;div class=&quot;row&quot;>
                    &lt;!-- START panel-->
                    &lt;div class=&quot;form-group  has-feedback mb-sm&quot;>
                        &lt;div class=&quot;col-sm-12&quot;>
                            &lt;div class=&quot;input-group mb-sm has-error-light&quot; ng-show=&quot;Message&quot;>
                                &lt;span class=&quot;input-group-addon bg-danger-light&quot;>&lt;em class=&quot;fa fa-warning &quot;>&lt;/em>&lt;/span>
                                &lt;label class=&quot;form-control m0&quot;>{{Message}}&lt;/label>
                            &lt;/div>
                        &lt;/div>
                    &lt;/div>
                    &lt;div class=&quot;form-group  has-feedback mb-sm&quot;>
                        &lt;div class=&quot;col-sm-12&quot;>
                            &lt;div class=&quot;input-group mb-sm has-error-light&quot; ng-show=&quot;chgPasswordMessage&quot;>
                                &lt;span class=&quot;input-group-addon bg-danger-light&quot;>&lt;em class=&quot;fa fa-warning &quot;>&lt;/em>&lt;/span>
                                &lt;label class=&quot;form-control m0&quot;>{{chgPasswordMessage}}&lt;/label>
                            &lt;/div>
                        &lt;/div>
                    &lt;/div>
                    &lt;div class=&quot;form-group  has-feedback mb-sm&quot;>
                        &lt;div class=&quot;col-sm-12&quot;>
                            &lt;label class=&quot;col-sm-4 control-label text-right text-sm&quot;>Username&lt;/label>
                            &lt;div class=&quot;col-sm-8&quot;>
                                &lt;div class=&quot;mb-sm&quot;>
                                    &lt;input type=&quot;text&quot; id=&quot;user_name&quot; name=&quot;username&quot; disabled class=&quot;form-control&quot; ng-model=&quot;UserPassword.UserName&quot; />
                                &lt;/div>
                            &lt;/div>
                        &lt;/div>
                    &lt;/div>
                    &lt;div class=&quot;form-group  has-feedback mb-sm&quot;>

                        &lt;div class=&quot;col-sm-12&quot;>
                            &lt;label class=&quot;col-sm-4 control-label text-right text-sm&quot;>Current Password&lt;/label>
                            &lt;div class=&quot;col-sm-8&quot;>
                                &lt;div class=&quot;mb-sm&quot;>
                                    &lt;input type=&quot;password&quot; placeholder=&quot;Current Password&quot; id=&quot;current_password&quot; name=&quot;currentpassword&quot; required=&quot;required&quot; class=&quot;form-control&quot; ng-model=&quot;UserPassword.CurrentPassword&quot; data-parsley-ui-enabled=&quot;false&quot; />
                                &lt;/div>
                                &lt;div class=&quot; alert bg-danger-light ht-lg p0 ph-sm pb-sm mb-sm&quot; name=&quot;currentpasswordRequired&quot; ng-show=&quot;!isCurrentPwdVal&quot;>
                                    &lt;em class=&quot;fa fa-warning pr-lg&quot;>&lt;/em>&lt;em>&lt;/em>Current Password is Required
                                &lt;/div>
                            &lt;/div>
                        &lt;/div>
                    &lt;/div>
                    &lt;div class=&quot;form-group  has-feedback mb-sm&quot;>
                        &lt;div class=&quot;col-sm-12&quot;>
                            &lt;label class=&quot;col-sm-4 control-label text-right text-sm&quot;>New Password&lt;/label>
                            &lt;div class=&quot;col-sm-8&quot;>
                                &lt;div class=&quot;mb-sm&quot;>
                                    &lt;input type=&quot;password&quot; placeholder=&quot;New Password&quot; id=&quot;new_password&quot; name=&quot;password_&quot; required=&quot;required&quot; class=&quot;form-control  info-light-tooltip&quot; ng-model=&quot;UserPassword.Password&quot; tooltip-html-unsafe=&quot;{{PasswordPolicy}}&quot; tooltip-trigger=&quot;focus&quot; tooltip-placement=&quot;top&quot; data-parsley-ui-enabled=&quot;false&quot; />
                                &lt;/div>
                                &lt;div class=&quot; alert bg-danger-light ht-lg p0 ph-sm pb-sm mb-sm&quot; name=&quot;newPasswordrequired&quot; ng-show=&quot;!isPwdVal&quot;>
                                    &lt;em class=&quot;fa fa-warning pr-lg&quot;>&lt;/em>&lt;em>&lt;/em>{{PwdMsg}}
                                &lt;/div>
                            &lt;/div>
                        &lt;/div>
                    &lt;/div>
                    &lt;div class=&quot;form-group  has-feedback mb-sm&quot;>
                        &lt;div class=&quot;col-sm-12&quot;>
                            &lt;label class=&quot;col-sm-4 control-label text-right text-sm&quot;>Confirm Password&lt;/label>
                            &lt;div class=&quot;col-sm-8&quot;>
                                &lt;div class=&quot;mb-sm&quot;>
                                    &lt;input type=&quot;password&quot; placeholder=&quot;Confirm Password&quot; id=&quot;confirm_password&quot; name=&quot;password2&quot; required=&quot;required&quot; class=&quot;form-control&quot; ng-model=&quot;UserPassword.ConfirmPassword&quot; data-parsley-ui-enabled=&quot;false&quot; />
                                &lt;/div>
                                &lt;div class=&quot; alert bg-danger-light ht-lg p0 ph-sm pb-sm mb-sm&quot; ng-show=&quot;!isPwd2Val&quot;>
                                    &lt;em class=&quot;fa fa-warning pr-lg&quot;>&lt;/em>&lt;em>&lt;/em>{{ConfirmMsg}}
                                &lt;/div>
                            &lt;/div>
                        &lt;/div>
                    &lt;/div>
                    &lt;!-- END panel-->
                &lt;/div>
            &lt;/div>
            &lt;div class=&quot;p0&quot; ng-show=&quot;isAdminResetPwd&quot;>
                &lt;div class=&quot;modal-header p0&quot;>
                    &lt;h4 id=&quot;modalPasswordChange&quot; class=&quot;modal-title text-left &quot; style=&quot;font-size:16px;&quot;>Update Personal Questions&lt;/h4>
                &lt;/div>
                &lt;div class=&quot;modal-body text-left&quot; style=&quot;padding-top:5px;&quot;>
                    &lt;div class=&quot;row&quot;>

                        &lt;div class=&quot;row&quot;>
                            &lt;div class=&quot;col-md-6&quot;>
                                &lt;div class=&quot;form-group mb-sm&quot;>
                                    &lt;label class=&quot;control-label&quot;>Question One&lt;/label>
                                    &lt;select name=&quot;question_one&quot; id=&quot;que_one&quot; class=&quot;form-control m-b&quot; data-parsley-group=&quot;step2&quot; ng-model=&quot;UserAccount.Question1&quot; ng-options=&quot;question.Question for question in Questions[0]&quot; required=&quot;required&quot; data-parsley-errors-messages-disabled data-parsley-ui-enabled=&quot;false&quot;>
                                        &lt;option value=&quot;&quot;>Choose a question...&lt;/option>
                                    &lt;/select>
                                &lt;/div>
                                &lt;div class=&quot;alert bg-danger-light ht-lg  p0 ph-sm&quot; ng-show=&quot;!isQ1Val&quot; style=&quot;margin-bottom:5px;&quot;>
                                    &lt;em class=&quot;fa fa-warning pr-lg&quot;>&lt;/em>&lt;em>&lt;/em>Choose question 1.
                                &lt;/div>
                                &lt;div class=&quot;form-group pt-lg mb-sm&quot;>
                                    &lt;input type=&quot;text&quot; name=&quot;answer_one&quot; data-parsley-group=&quot;step2&quot; placeholder=&quot;Answer to question one&quot; required=&quot;required&quot; class=&quot;form-control&quot; ng-model=&quot;UserAccount.Answer1&quot; data-parsley-errors-messages-disabled data-parsley-ui-enabled=&quot;false&quot; />
                                &lt;/div>
                                &lt;div class=&quot;alert bg-danger-light ht-lg  p0 ph-sm&quot; ng-show=&quot;!isA1Val&quot; style=&quot;margin-bottom:5px;&quot;>
                                    &lt;em class=&quot;fa fa-warning pr-lg&quot;>&lt;/em>&lt;em>&lt;/em>Answer question 1.
                                &lt;/div>
                            &lt;/div>
                            &lt;div class=&quot;col-md-6&quot;>
                                &lt;div class=&quot;form-group mb-sm&quot;>
                                    &lt;label class=&quot;control-label&quot;>Question Two&lt;/label>
                                    &lt;select name=&quot;question_two&quot; id=&quot;que_two&quot; class=&quot;form-control m-b&quot; data-parsley-group=&quot;step2&quot; ng-model=&quot;UserAccount.Question2&quot; ng-options=&quot;question.Question for question in Questions[1]&quot; required=&quot;required&quot; data-parsley-errors-messages-disabled data-parsley-ui-enabled=&quot;false&quot;>
                                        &lt;option value=&quot;&quot;>Choose a question...&lt;/option>
                                    &lt;/select>
                                &lt;/div>
                                &lt;div class=&quot;alert bg-danger-light ht-lg  p0 ph-sm&quot; ng-show=&quot;!isQ2Val&quot; style=&quot;margin-bottom:5px;&quot;>
                                    &lt;em class=&quot;fa fa-warning pr-lg&quot;>&lt;/em>&lt;em>&lt;/em>Choose question 2.
                                &lt;/div>
                                &lt;div class=&quot;form-group pt-lg mb-sm&quot;>
                                    &lt;input type=&quot;text&quot; name=&quot;answer_two&quot; data-parsley-group=&quot;step2&quot; placeholder=&quot;Answer to question two&quot; required=&quot;required&quot; class=&quot;form-control&quot; ng-model=&quot;UserAccount.Answer2&quot; data-parsley-errors-messages-disabled data-parsley-ui-enabled=&quot;false&quot; />
                                &lt;/div>
                                &lt;div class=&quot;alert bg-danger-light ht-lg  p0 ph-sm&quot; ng-show=&quot;!isA2Val&quot; style=&quot;margin-bottom:5px;&quot;>
                                    &lt;em class=&quot;fa fa-warning pr-lg&quot;>&lt;/em>&lt;em>&lt;/em>Answer question 2.
                                &lt;/div>
                            &lt;/div>
                        &lt;/div>
                    &lt;/div>
                &lt;/div>
            &lt;/div>
            &lt;div class=&quot;modal-footer pb0&quot;>&lt;button ng-click=&quot;ok()&quot; id=&quot;chg_pwd&quot; class=&quot;btn btn-primary&quot;>Change Password&lt;/button>&lt;button ng-click=&quot;cancel()&quot; id=&quot;cancel&quot; class=&quot;btn btn-warning&quot;>Cancel&lt;/button>&lt;/div>
        &lt;/div>

        &lt;div class=&quot;p text-center&quot; style=&quot;font-size:12px&quot; ng-show=&quot;!isSuccess&quot;>
            &lt;span>&amp;copy;&lt;/span>
            &lt;span ng-bind=&quot;app.year&quot;>&lt;/span>
            &lt;span>-&lt;/span>
            &lt;span ng-bind=&quot;app.name&quot;>&lt;/span>
            &lt;br />
            &lt;span ng-bind=&quot;app.description&quot;>&lt;/span>
        &lt;/div>
        &lt;!--&lt;div style=&quot;font-size:14px;padding-top:5px;&quot; ng-show=&quot;isSuccess&quot;>
            &lt;div style=&quot;padding-top:5px;&quot;>
                &lt;span style=&quot;padding-top:5px;&quot;>Password details updated successfully.&lt;/span>
                &lt;div class=&quot;modal-footer pb0&quot;>&lt;button ng-click=&quot;cancel()&quot; class=&quot;btn btn-primary&quot;>Ok&lt;/button>&lt;/div>
            &lt;/div>
        &lt;/div>-->
    
    


            
                ©
                2024
                -
                Conduent Business Services, LLC.
                
                Conduent Loan Manager User Setup and Application Configuration Portal
                
                v.1.0.21.0
            
    
   
</value>
      <webElementGuid>aa72bd95-8deb-4cb8-ab80-bf7cec44280f</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>/html[@class=&quot;js flexbox flexboxlegacy canvas canvastext webgl no-touch geolocation postmessage no-websqldatabase indexeddb hashchange history draganddrop websockets rgba hsla multiplebgs backgroundsize borderimage borderradius boxshadow textshadow opacity cssanimations csscolumns cssgradients cssreflections csstransforms csstransforms3d csstransitions fontface generatedcontent video audio localstorage sessionstorage webworkers no-applicationcache svg inlinesvg smil svgclippaths ng-scope js flexbox flexboxlegacy canvas canvastext webgl no-touch geolocation postmessage no-websqldatabase indexeddb hashchange history draganddrop websockets rgba hsla multiplebgs backgroundsize borderimage borderradius boxshadow textshadow opacity cssanimations csscolumns cssgradients cssreflections csstransforms csstransforms3d csstransitions fontface generatedcontent video audio localstorage sessionstorage webworkers no-applicationcache svg inlinesvg smil svgclippaths&quot;]/body[@class=&quot;layout-fixed home_page_bg&quot;]/div[@class=&quot;wrapper ng-scope ng-fadeInDown&quot;]/div[@class=&quot;ng-zoomBackDown ng-fluid ng-scope&quot;]/div[@class=&quot;unwrap ng-scope&quot;]/div[@class=&quot;bg-cover&quot;]</value>
      <webElementGuid>39287124-0fd2-43f5-b6d4-c5cea8d3cf6d</webElementGuid>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//div/div/div/div</value>
      <webElementGuid>a40b1b1f-0ec9-4e17-b08f-940e3e78b2e2</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:customAttributes</name>
      <type>Main</type>
      <value>//div[(text() = '
        
            
                Admin Portal
            
            
            Conduent Loan Manager User Setup and Application Configuration
            This portal provides you with access to our users and applications
        
        
            
            
                
                    Sign In
                
                
                    Sign In to manage users and configure applications.
                    
                        
                        
                        
                            
                            
                        
                        
                            User name is required.
                        
                        
                            
                            
                        
                        
                            Password is required.
                        
                        
                            
                                
                                    
                                    Remember Me
                                
                            
                            
                                Forgot your password?
                            
                        
                        Login
                    

                    
                
            
            
            
    
    

        &lt;div class=&quot;text-center text-csr-heading&quot; style=&quot;margin-top:10px;&quot; ng-show=&quot;!isSuccess&quot;>
            &lt;!--&lt;div class=&quot;empty-wrapper mt-lg&quot;>
              &lt;!--  &lt;span class=&quot;fa fa-users fa-3x&quot;>&lt;/span>&lt;br />&lt;span>CSR Portal&lt;/span>

            &lt;/div>-->
            &lt;img src=&quot;img/conduent_logo.png&quot; id=&quot;img&quot; alt=&quot;Logo&quot; class=&quot;m0&quot; style=&quot;width:200px&quot;/>
            &lt;h3 style=&quot;margin-top:5px; margin-bottom:5px;&quot;>Loan Manager&lt;/h3>
            &lt;p>Access customers loan information for maintenance and service&lt;/p>
        &lt;/div>

        &lt;div class=&quot;panel-body pb0 pt0&quot; ng-show=&quot;!isSuccess&quot;>
            &lt;div class=&quot;modal-header p0&quot;>
                &lt;h4 id=&quot;modalPasswordChange&quot; class=&quot;modal-title text-left &quot; style=&quot;font-size:16px;&quot;>Password Change Required!&lt;/h4>
            &lt;/div>
            &lt;div class=&quot;modal-body text-left pb0&quot;>Please enter your current password and your desired new password.&lt;/div>
            &lt;div class=&quot;modal-body text-left pb0 pt0&quot; style=&quot;padding-bottom:5px;&quot;>
                &lt;div class=&quot;row&quot;>
                    &lt;!-- START panel-->
                    &lt;div class=&quot;form-group  has-feedback mb-sm&quot;>
                        &lt;div class=&quot;col-sm-12&quot;>
                            &lt;div class=&quot;input-group mb-sm has-error-light&quot; ng-show=&quot;Message&quot;>
                                &lt;span class=&quot;input-group-addon bg-danger-light&quot;>&lt;em class=&quot;fa fa-warning &quot;>&lt;/em>&lt;/span>
                                &lt;label class=&quot;form-control m0&quot;>{{Message}}&lt;/label>
                            &lt;/div>
                        &lt;/div>
                    &lt;/div>
                    &lt;div class=&quot;form-group  has-feedback mb-sm&quot;>
                        &lt;div class=&quot;col-sm-12&quot;>
                            &lt;div class=&quot;input-group mb-sm has-error-light&quot; ng-show=&quot;chgPasswordMessage&quot;>
                                &lt;span class=&quot;input-group-addon bg-danger-light&quot;>&lt;em class=&quot;fa fa-warning &quot;>&lt;/em>&lt;/span>
                                &lt;label class=&quot;form-control m0&quot;>{{chgPasswordMessage}}&lt;/label>
                            &lt;/div>
                        &lt;/div>
                    &lt;/div>
                    &lt;div class=&quot;form-group  has-feedback mb-sm&quot;>
                        &lt;div class=&quot;col-sm-12&quot;>
                            &lt;label class=&quot;col-sm-4 control-label text-right text-sm&quot;>Username&lt;/label>
                            &lt;div class=&quot;col-sm-8&quot;>
                                &lt;div class=&quot;mb-sm&quot;>
                                    &lt;input type=&quot;text&quot; id=&quot;user_name&quot; name=&quot;username&quot; disabled class=&quot;form-control&quot; ng-model=&quot;UserPassword.UserName&quot; />
                                &lt;/div>
                            &lt;/div>
                        &lt;/div>
                    &lt;/div>
                    &lt;div class=&quot;form-group  has-feedback mb-sm&quot;>

                        &lt;div class=&quot;col-sm-12&quot;>
                            &lt;label class=&quot;col-sm-4 control-label text-right text-sm&quot;>Current Password&lt;/label>
                            &lt;div class=&quot;col-sm-8&quot;>
                                &lt;div class=&quot;mb-sm&quot;>
                                    &lt;input type=&quot;password&quot; placeholder=&quot;Current Password&quot; id=&quot;current_password&quot; name=&quot;currentpassword&quot; required=&quot;required&quot; class=&quot;form-control&quot; ng-model=&quot;UserPassword.CurrentPassword&quot; data-parsley-ui-enabled=&quot;false&quot; />
                                &lt;/div>
                                &lt;div class=&quot; alert bg-danger-light ht-lg p0 ph-sm pb-sm mb-sm&quot; name=&quot;currentpasswordRequired&quot; ng-show=&quot;!isCurrentPwdVal&quot;>
                                    &lt;em class=&quot;fa fa-warning pr-lg&quot;>&lt;/em>&lt;em>&lt;/em>Current Password is Required
                                &lt;/div>
                            &lt;/div>
                        &lt;/div>
                    &lt;/div>
                    &lt;div class=&quot;form-group  has-feedback mb-sm&quot;>
                        &lt;div class=&quot;col-sm-12&quot;>
                            &lt;label class=&quot;col-sm-4 control-label text-right text-sm&quot;>New Password&lt;/label>
                            &lt;div class=&quot;col-sm-8&quot;>
                                &lt;div class=&quot;mb-sm&quot;>
                                    &lt;input type=&quot;password&quot; placeholder=&quot;New Password&quot; id=&quot;new_password&quot; name=&quot;password_&quot; required=&quot;required&quot; class=&quot;form-control  info-light-tooltip&quot; ng-model=&quot;UserPassword.Password&quot; tooltip-html-unsafe=&quot;{{PasswordPolicy}}&quot; tooltip-trigger=&quot;focus&quot; tooltip-placement=&quot;top&quot; data-parsley-ui-enabled=&quot;false&quot; />
                                &lt;/div>
                                &lt;div class=&quot; alert bg-danger-light ht-lg p0 ph-sm pb-sm mb-sm&quot; name=&quot;newPasswordrequired&quot; ng-show=&quot;!isPwdVal&quot;>
                                    &lt;em class=&quot;fa fa-warning pr-lg&quot;>&lt;/em>&lt;em>&lt;/em>{{PwdMsg}}
                                &lt;/div>
                            &lt;/div>
                        &lt;/div>
                    &lt;/div>
                    &lt;div class=&quot;form-group  has-feedback mb-sm&quot;>
                        &lt;div class=&quot;col-sm-12&quot;>
                            &lt;label class=&quot;col-sm-4 control-label text-right text-sm&quot;>Confirm Password&lt;/label>
                            &lt;div class=&quot;col-sm-8&quot;>
                                &lt;div class=&quot;mb-sm&quot;>
                                    &lt;input type=&quot;password&quot; placeholder=&quot;Confirm Password&quot; id=&quot;confirm_password&quot; name=&quot;password2&quot; required=&quot;required&quot; class=&quot;form-control&quot; ng-model=&quot;UserPassword.ConfirmPassword&quot; data-parsley-ui-enabled=&quot;false&quot; />
                                &lt;/div>
                                &lt;div class=&quot; alert bg-danger-light ht-lg p0 ph-sm pb-sm mb-sm&quot; ng-show=&quot;!isPwd2Val&quot;>
                                    &lt;em class=&quot;fa fa-warning pr-lg&quot;>&lt;/em>&lt;em>&lt;/em>{{ConfirmMsg}}
                                &lt;/div>
                            &lt;/div>
                        &lt;/div>
                    &lt;/div>
                    &lt;!-- END panel-->
                &lt;/div>
            &lt;/div>
            &lt;div class=&quot;p0&quot; ng-show=&quot;isAdminResetPwd&quot;>
                &lt;div class=&quot;modal-header p0&quot;>
                    &lt;h4 id=&quot;modalPasswordChange&quot; class=&quot;modal-title text-left &quot; style=&quot;font-size:16px;&quot;>Update Personal Questions&lt;/h4>
                &lt;/div>
                &lt;div class=&quot;modal-body text-left&quot; style=&quot;padding-top:5px;&quot;>
                    &lt;div class=&quot;row&quot;>

                        &lt;div class=&quot;row&quot;>
                            &lt;div class=&quot;col-md-6&quot;>
                                &lt;div class=&quot;form-group mb-sm&quot;>
                                    &lt;label class=&quot;control-label&quot;>Question One&lt;/label>
                                    &lt;select name=&quot;question_one&quot; id=&quot;que_one&quot; class=&quot;form-control m-b&quot; data-parsley-group=&quot;step2&quot; ng-model=&quot;UserAccount.Question1&quot; ng-options=&quot;question.Question for question in Questions[0]&quot; required=&quot;required&quot; data-parsley-errors-messages-disabled data-parsley-ui-enabled=&quot;false&quot;>
                                        &lt;option value=&quot;&quot;>Choose a question...&lt;/option>
                                    &lt;/select>
                                &lt;/div>
                                &lt;div class=&quot;alert bg-danger-light ht-lg  p0 ph-sm&quot; ng-show=&quot;!isQ1Val&quot; style=&quot;margin-bottom:5px;&quot;>
                                    &lt;em class=&quot;fa fa-warning pr-lg&quot;>&lt;/em>&lt;em>&lt;/em>Choose question 1.
                                &lt;/div>
                                &lt;div class=&quot;form-group pt-lg mb-sm&quot;>
                                    &lt;input type=&quot;text&quot; name=&quot;answer_one&quot; data-parsley-group=&quot;step2&quot; placeholder=&quot;Answer to question one&quot; required=&quot;required&quot; class=&quot;form-control&quot; ng-model=&quot;UserAccount.Answer1&quot; data-parsley-errors-messages-disabled data-parsley-ui-enabled=&quot;false&quot; />
                                &lt;/div>
                                &lt;div class=&quot;alert bg-danger-light ht-lg  p0 ph-sm&quot; ng-show=&quot;!isA1Val&quot; style=&quot;margin-bottom:5px;&quot;>
                                    &lt;em class=&quot;fa fa-warning pr-lg&quot;>&lt;/em>&lt;em>&lt;/em>Answer question 1.
                                &lt;/div>
                            &lt;/div>
                            &lt;div class=&quot;col-md-6&quot;>
                                &lt;div class=&quot;form-group mb-sm&quot;>
                                    &lt;label class=&quot;control-label&quot;>Question Two&lt;/label>
                                    &lt;select name=&quot;question_two&quot; id=&quot;que_two&quot; class=&quot;form-control m-b&quot; data-parsley-group=&quot;step2&quot; ng-model=&quot;UserAccount.Question2&quot; ng-options=&quot;question.Question for question in Questions[1]&quot; required=&quot;required&quot; data-parsley-errors-messages-disabled data-parsley-ui-enabled=&quot;false&quot;>
                                        &lt;option value=&quot;&quot;>Choose a question...&lt;/option>
                                    &lt;/select>
                                &lt;/div>
                                &lt;div class=&quot;alert bg-danger-light ht-lg  p0 ph-sm&quot; ng-show=&quot;!isQ2Val&quot; style=&quot;margin-bottom:5px;&quot;>
                                    &lt;em class=&quot;fa fa-warning pr-lg&quot;>&lt;/em>&lt;em>&lt;/em>Choose question 2.
                                &lt;/div>
                                &lt;div class=&quot;form-group pt-lg mb-sm&quot;>
                                    &lt;input type=&quot;text&quot; name=&quot;answer_two&quot; data-parsley-group=&quot;step2&quot; placeholder=&quot;Answer to question two&quot; required=&quot;required&quot; class=&quot;form-control&quot; ng-model=&quot;UserAccount.Answer2&quot; data-parsley-errors-messages-disabled data-parsley-ui-enabled=&quot;false&quot; />
                                &lt;/div>
                                &lt;div class=&quot;alert bg-danger-light ht-lg  p0 ph-sm&quot; ng-show=&quot;!isA2Val&quot; style=&quot;margin-bottom:5px;&quot;>
                                    &lt;em class=&quot;fa fa-warning pr-lg&quot;>&lt;/em>&lt;em>&lt;/em>Answer question 2.
                                &lt;/div>
                            &lt;/div>
                        &lt;/div>
                    &lt;/div>
                &lt;/div>
            &lt;/div>
            &lt;div class=&quot;modal-footer pb0&quot;>&lt;button ng-click=&quot;ok()&quot; id=&quot;chg_pwd&quot; class=&quot;btn btn-primary&quot;>Change Password&lt;/button>&lt;button ng-click=&quot;cancel()&quot; id=&quot;cancel&quot; class=&quot;btn btn-warning&quot;>Cancel&lt;/button>&lt;/div>
        &lt;/div>

        &lt;div class=&quot;p text-center&quot; style=&quot;font-size:12px&quot; ng-show=&quot;!isSuccess&quot;>
            &lt;span>&amp;copy;&lt;/span>
            &lt;span ng-bind=&quot;app.year&quot;>&lt;/span>
            &lt;span>-&lt;/span>
            &lt;span ng-bind=&quot;app.name&quot;>&lt;/span>
            &lt;br />
            &lt;span ng-bind=&quot;app.description&quot;>&lt;/span>
        &lt;/div>
        &lt;!--&lt;div style=&quot;font-size:14px;padding-top:5px;&quot; ng-show=&quot;isSuccess&quot;>
            &lt;div style=&quot;padding-top:5px;&quot;>
                &lt;span style=&quot;padding-top:5px;&quot;>Password details updated successfully.&lt;/span>
                &lt;div class=&quot;modal-footer pb0&quot;>&lt;button ng-click=&quot;cancel()&quot; class=&quot;btn btn-primary&quot;>Ok&lt;/button>&lt;/div>
            &lt;/div>
        &lt;/div>-->
    
    


            
                ©
                2024
                -
                Conduent Business Services, LLC.
                
                Conduent Loan Manager User Setup and Application Configuration Portal
                
                v.1.0.21.0
            
    
   
' or . = '
        
            
                Admin Portal
            
            
            Conduent Loan Manager User Setup and Application Configuration
            This portal provides you with access to our users and applications
        
        
            
            
                
                    Sign In
                
                
                    Sign In to manage users and configure applications.
                    
                        
                        
                        
                            
                            
                        
                        
                            User name is required.
                        
                        
                            
                            
                        
                        
                            Password is required.
                        
                        
                            
                                
                                    
                                    Remember Me
                                
                            
                            
                                Forgot your password?
                            
                        
                        Login
                    

                    
                
            
            
            
    
    

        &lt;div class=&quot;text-center text-csr-heading&quot; style=&quot;margin-top:10px;&quot; ng-show=&quot;!isSuccess&quot;>
            &lt;!--&lt;div class=&quot;empty-wrapper mt-lg&quot;>
              &lt;!--  &lt;span class=&quot;fa fa-users fa-3x&quot;>&lt;/span>&lt;br />&lt;span>CSR Portal&lt;/span>

            &lt;/div>-->
            &lt;img src=&quot;img/conduent_logo.png&quot; id=&quot;img&quot; alt=&quot;Logo&quot; class=&quot;m0&quot; style=&quot;width:200px&quot;/>
            &lt;h3 style=&quot;margin-top:5px; margin-bottom:5px;&quot;>Loan Manager&lt;/h3>
            &lt;p>Access customers loan information for maintenance and service&lt;/p>
        &lt;/div>

        &lt;div class=&quot;panel-body pb0 pt0&quot; ng-show=&quot;!isSuccess&quot;>
            &lt;div class=&quot;modal-header p0&quot;>
                &lt;h4 id=&quot;modalPasswordChange&quot; class=&quot;modal-title text-left &quot; style=&quot;font-size:16px;&quot;>Password Change Required!&lt;/h4>
            &lt;/div>
            &lt;div class=&quot;modal-body text-left pb0&quot;>Please enter your current password and your desired new password.&lt;/div>
            &lt;div class=&quot;modal-body text-left pb0 pt0&quot; style=&quot;padding-bottom:5px;&quot;>
                &lt;div class=&quot;row&quot;>
                    &lt;!-- START panel-->
                    &lt;div class=&quot;form-group  has-feedback mb-sm&quot;>
                        &lt;div class=&quot;col-sm-12&quot;>
                            &lt;div class=&quot;input-group mb-sm has-error-light&quot; ng-show=&quot;Message&quot;>
                                &lt;span class=&quot;input-group-addon bg-danger-light&quot;>&lt;em class=&quot;fa fa-warning &quot;>&lt;/em>&lt;/span>
                                &lt;label class=&quot;form-control m0&quot;>{{Message}}&lt;/label>
                            &lt;/div>
                        &lt;/div>
                    &lt;/div>
                    &lt;div class=&quot;form-group  has-feedback mb-sm&quot;>
                        &lt;div class=&quot;col-sm-12&quot;>
                            &lt;div class=&quot;input-group mb-sm has-error-light&quot; ng-show=&quot;chgPasswordMessage&quot;>
                                &lt;span class=&quot;input-group-addon bg-danger-light&quot;>&lt;em class=&quot;fa fa-warning &quot;>&lt;/em>&lt;/span>
                                &lt;label class=&quot;form-control m0&quot;>{{chgPasswordMessage}}&lt;/label>
                            &lt;/div>
                        &lt;/div>
                    &lt;/div>
                    &lt;div class=&quot;form-group  has-feedback mb-sm&quot;>
                        &lt;div class=&quot;col-sm-12&quot;>
                            &lt;label class=&quot;col-sm-4 control-label text-right text-sm&quot;>Username&lt;/label>
                            &lt;div class=&quot;col-sm-8&quot;>
                                &lt;div class=&quot;mb-sm&quot;>
                                    &lt;input type=&quot;text&quot; id=&quot;user_name&quot; name=&quot;username&quot; disabled class=&quot;form-control&quot; ng-model=&quot;UserPassword.UserName&quot; />
                                &lt;/div>
                            &lt;/div>
                        &lt;/div>
                    &lt;/div>
                    &lt;div class=&quot;form-group  has-feedback mb-sm&quot;>

                        &lt;div class=&quot;col-sm-12&quot;>
                            &lt;label class=&quot;col-sm-4 control-label text-right text-sm&quot;>Current Password&lt;/label>
                            &lt;div class=&quot;col-sm-8&quot;>
                                &lt;div class=&quot;mb-sm&quot;>
                                    &lt;input type=&quot;password&quot; placeholder=&quot;Current Password&quot; id=&quot;current_password&quot; name=&quot;currentpassword&quot; required=&quot;required&quot; class=&quot;form-control&quot; ng-model=&quot;UserPassword.CurrentPassword&quot; data-parsley-ui-enabled=&quot;false&quot; />
                                &lt;/div>
                                &lt;div class=&quot; alert bg-danger-light ht-lg p0 ph-sm pb-sm mb-sm&quot; name=&quot;currentpasswordRequired&quot; ng-show=&quot;!isCurrentPwdVal&quot;>
                                    &lt;em class=&quot;fa fa-warning pr-lg&quot;>&lt;/em>&lt;em>&lt;/em>Current Password is Required
                                &lt;/div>
                            &lt;/div>
                        &lt;/div>
                    &lt;/div>
                    &lt;div class=&quot;form-group  has-feedback mb-sm&quot;>
                        &lt;div class=&quot;col-sm-12&quot;>
                            &lt;label class=&quot;col-sm-4 control-label text-right text-sm&quot;>New Password&lt;/label>
                            &lt;div class=&quot;col-sm-8&quot;>
                                &lt;div class=&quot;mb-sm&quot;>
                                    &lt;input type=&quot;password&quot; placeholder=&quot;New Password&quot; id=&quot;new_password&quot; name=&quot;password_&quot; required=&quot;required&quot; class=&quot;form-control  info-light-tooltip&quot; ng-model=&quot;UserPassword.Password&quot; tooltip-html-unsafe=&quot;{{PasswordPolicy}}&quot; tooltip-trigger=&quot;focus&quot; tooltip-placement=&quot;top&quot; data-parsley-ui-enabled=&quot;false&quot; />
                                &lt;/div>
                                &lt;div class=&quot; alert bg-danger-light ht-lg p0 ph-sm pb-sm mb-sm&quot; name=&quot;newPasswordrequired&quot; ng-show=&quot;!isPwdVal&quot;>
                                    &lt;em class=&quot;fa fa-warning pr-lg&quot;>&lt;/em>&lt;em>&lt;/em>{{PwdMsg}}
                                &lt;/div>
                            &lt;/div>
                        &lt;/div>
                    &lt;/div>
                    &lt;div class=&quot;form-group  has-feedback mb-sm&quot;>
                        &lt;div class=&quot;col-sm-12&quot;>
                            &lt;label class=&quot;col-sm-4 control-label text-right text-sm&quot;>Confirm Password&lt;/label>
                            &lt;div class=&quot;col-sm-8&quot;>
                                &lt;div class=&quot;mb-sm&quot;>
                                    &lt;input type=&quot;password&quot; placeholder=&quot;Confirm Password&quot; id=&quot;confirm_password&quot; name=&quot;password2&quot; required=&quot;required&quot; class=&quot;form-control&quot; ng-model=&quot;UserPassword.ConfirmPassword&quot; data-parsley-ui-enabled=&quot;false&quot; />
                                &lt;/div>
                                &lt;div class=&quot; alert bg-danger-light ht-lg p0 ph-sm pb-sm mb-sm&quot; ng-show=&quot;!isPwd2Val&quot;>
                                    &lt;em class=&quot;fa fa-warning pr-lg&quot;>&lt;/em>&lt;em>&lt;/em>{{ConfirmMsg}}
                                &lt;/div>
                            &lt;/div>
                        &lt;/div>
                    &lt;/div>
                    &lt;!-- END panel-->
                &lt;/div>
            &lt;/div>
            &lt;div class=&quot;p0&quot; ng-show=&quot;isAdminResetPwd&quot;>
                &lt;div class=&quot;modal-header p0&quot;>
                    &lt;h4 id=&quot;modalPasswordChange&quot; class=&quot;modal-title text-left &quot; style=&quot;font-size:16px;&quot;>Update Personal Questions&lt;/h4>
                &lt;/div>
                &lt;div class=&quot;modal-body text-left&quot; style=&quot;padding-top:5px;&quot;>
                    &lt;div class=&quot;row&quot;>

                        &lt;div class=&quot;row&quot;>
                            &lt;div class=&quot;col-md-6&quot;>
                                &lt;div class=&quot;form-group mb-sm&quot;>
                                    &lt;label class=&quot;control-label&quot;>Question One&lt;/label>
                                    &lt;select name=&quot;question_one&quot; id=&quot;que_one&quot; class=&quot;form-control m-b&quot; data-parsley-group=&quot;step2&quot; ng-model=&quot;UserAccount.Question1&quot; ng-options=&quot;question.Question for question in Questions[0]&quot; required=&quot;required&quot; data-parsley-errors-messages-disabled data-parsley-ui-enabled=&quot;false&quot;>
                                        &lt;option value=&quot;&quot;>Choose a question...&lt;/option>
                                    &lt;/select>
                                &lt;/div>
                                &lt;div class=&quot;alert bg-danger-light ht-lg  p0 ph-sm&quot; ng-show=&quot;!isQ1Val&quot; style=&quot;margin-bottom:5px;&quot;>
                                    &lt;em class=&quot;fa fa-warning pr-lg&quot;>&lt;/em>&lt;em>&lt;/em>Choose question 1.
                                &lt;/div>
                                &lt;div class=&quot;form-group pt-lg mb-sm&quot;>
                                    &lt;input type=&quot;text&quot; name=&quot;answer_one&quot; data-parsley-group=&quot;step2&quot; placeholder=&quot;Answer to question one&quot; required=&quot;required&quot; class=&quot;form-control&quot; ng-model=&quot;UserAccount.Answer1&quot; data-parsley-errors-messages-disabled data-parsley-ui-enabled=&quot;false&quot; />
                                &lt;/div>
                                &lt;div class=&quot;alert bg-danger-light ht-lg  p0 ph-sm&quot; ng-show=&quot;!isA1Val&quot; style=&quot;margin-bottom:5px;&quot;>
                                    &lt;em class=&quot;fa fa-warning pr-lg&quot;>&lt;/em>&lt;em>&lt;/em>Answer question 1.
                                &lt;/div>
                            &lt;/div>
                            &lt;div class=&quot;col-md-6&quot;>
                                &lt;div class=&quot;form-group mb-sm&quot;>
                                    &lt;label class=&quot;control-label&quot;>Question Two&lt;/label>
                                    &lt;select name=&quot;question_two&quot; id=&quot;que_two&quot; class=&quot;form-control m-b&quot; data-parsley-group=&quot;step2&quot; ng-model=&quot;UserAccount.Question2&quot; ng-options=&quot;question.Question for question in Questions[1]&quot; required=&quot;required&quot; data-parsley-errors-messages-disabled data-parsley-ui-enabled=&quot;false&quot;>
                                        &lt;option value=&quot;&quot;>Choose a question...&lt;/option>
                                    &lt;/select>
                                &lt;/div>
                                &lt;div class=&quot;alert bg-danger-light ht-lg  p0 ph-sm&quot; ng-show=&quot;!isQ2Val&quot; style=&quot;margin-bottom:5px;&quot;>
                                    &lt;em class=&quot;fa fa-warning pr-lg&quot;>&lt;/em>&lt;em>&lt;/em>Choose question 2.
                                &lt;/div>
                                &lt;div class=&quot;form-group pt-lg mb-sm&quot;>
                                    &lt;input type=&quot;text&quot; name=&quot;answer_two&quot; data-parsley-group=&quot;step2&quot; placeholder=&quot;Answer to question two&quot; required=&quot;required&quot; class=&quot;form-control&quot; ng-model=&quot;UserAccount.Answer2&quot; data-parsley-errors-messages-disabled data-parsley-ui-enabled=&quot;false&quot; />
                                &lt;/div>
                                &lt;div class=&quot;alert bg-danger-light ht-lg  p0 ph-sm&quot; ng-show=&quot;!isA2Val&quot; style=&quot;margin-bottom:5px;&quot;>
                                    &lt;em class=&quot;fa fa-warning pr-lg&quot;>&lt;/em>&lt;em>&lt;/em>Answer question 2.
                                &lt;/div>
                            &lt;/div>
                        &lt;/div>
                    &lt;/div>
                &lt;/div>
            &lt;/div>
            &lt;div class=&quot;modal-footer pb0&quot;>&lt;button ng-click=&quot;ok()&quot; id=&quot;chg_pwd&quot; class=&quot;btn btn-primary&quot;>Change Password&lt;/button>&lt;button ng-click=&quot;cancel()&quot; id=&quot;cancel&quot; class=&quot;btn btn-warning&quot;>Cancel&lt;/button>&lt;/div>
        &lt;/div>

        &lt;div class=&quot;p text-center&quot; style=&quot;font-size:12px&quot; ng-show=&quot;!isSuccess&quot;>
            &lt;span>&amp;copy;&lt;/span>
            &lt;span ng-bind=&quot;app.year&quot;>&lt;/span>
            &lt;span>-&lt;/span>
            &lt;span ng-bind=&quot;app.name&quot;>&lt;/span>
            &lt;br />
            &lt;span ng-bind=&quot;app.description&quot;>&lt;/span>
        &lt;/div>
        &lt;!--&lt;div style=&quot;font-size:14px;padding-top:5px;&quot; ng-show=&quot;isSuccess&quot;>
            &lt;div style=&quot;padding-top:5px;&quot;>
                &lt;span style=&quot;padding-top:5px;&quot;>Password details updated successfully.&lt;/span>
                &lt;div class=&quot;modal-footer pb0&quot;>&lt;button ng-click=&quot;cancel()&quot; class=&quot;btn btn-primary&quot;>Ok&lt;/button>&lt;/div>
            &lt;/div>
        &lt;/div>-->
    
    


            
                ©
                2024
                -
                Conduent Business Services, LLC.
                
                Conduent Loan Manager User Setup and Application Configuration Portal
                
                v.1.0.21.0
            
    
   
')]</value>
      <webElementGuid>7e8256b0-195a-412d-8d46-c8258504e313</webElementGuid>
   </webElementXpaths>
</WebElementEntity>
