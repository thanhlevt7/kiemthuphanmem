<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>div_Tm kim  ng nhp ti khonNhp email v mt kh_3b2d13</name>
   <tag></tag>
   <elementGuidId>842c0700-d134-41ef-ae4e-6dad3501c892</elementGuidId>
   <selectorCollection>
      <entry>
         <key>XPATH</key>
         <value>//header[@id='site-header']/div[2]/div/div/div[4]</value>
      </entry>
      <entry>
         <key>CSS</key>
         <value>div.header-wrap-icon</value>
      </entry>
   </selectorCollection>
   <selectorMethod>XPATH</selectorMethod>
   <useRalativeImagePath>true</useRalativeImagePath>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>div</value>
      <webElementGuid>02d475cd-6f7b-4fbf-b4de-c62562cb2edc</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>class</name>
      <type>Main</type>
      <value>header-wrap-icon</value>
      <webElementGuid>30ff360f-cb8f-44ac-864f-c94101847051</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>
						
							
								
									  
									
										  
									
								
							
							
								
									  
								
								
									Tìm kiếm
									
										
											
											
											
												
											
										
										
											
										
									
								
							
						
						
							
								
									
									
										  
									
								
							


	
		
			
				
					Đăng nhập tài khoản
					Nhập email và mật khẩu của bạn:
				
				
					



					

					
						
						Nhập email hoặc số điện thoại
					
					
						
						
						Mật khẩu						
										
					
					
            Đăng nhập Google
            Đăng nhập Facebook
            
          
					Đăng nhập
					
					
grecaptcha.ready(function() {grecaptcha.execute('6LdD18MUAAAAAHqKl3Avv8W-tREL6LangePxQLM-', {action: 'submit'}).then(function(token) {document.getElementById('16330fa71c994363bb3bc902339e86ec').value = token;});});
					
						Khách hàng mới? 
							Tạo tài khoản
						
						Quên mật khẩu? 
							Khôi phục mật khẩu
						
					
				
			
		
		
			
			
				Khôi phục mật khẩu
			
			
				
					
					Khôi phục bằng email
				
				
					
					Khôi phục bằng số điện thoại
				
			
			

									
					
						
						
							
						
						
							
						
					
					
						
						
						
							
						
						
							
						
					
					
						function On_PhoneAuthRecaptchaCallback(token)
						{
							var frm = $('#phone_auth_recaptcha');

							$.ajax({
								type: &quot;POST&quot;,
								url: '/phone_auth/send_verify_code',
								data: frm.serialize(),
								dataType : &quot;json&quot;,
								success: function(data, textStatus, jqXHR)
								{
									if(data &amp;&amp; data.token)
									{
										if($('#session_info').length > 0){
											$('#session_info').val(data.token);
											$('#phone_auth_recaptcha').hide();
											$('.otpcode').show();
										}
									}
								},
								error: function(jqXHR, textStatus, errorThrown)
								{

								}
							});
						}
									
			
			

				
					



					

					
						
						Email
						
					
					Khôi phục

grecaptcha.ready(function() {grecaptcha.execute('6LdD18MUAAAAAHqKl3Avv8W-tREL6LangePxQLM-', {action: 'submit'}).then(function(token) {document.getElementById('353e293534434762bdf3734429e2d5f1').value = token;});});					
						Bạn đã nhớ mật khẩu? 
							Trở về đăng nhập
						
					
				
			
			
		
	


	.phoneCheckShow {
		display: none;
	}
	.otpcode{
		display: none;
	}



	$('.optionCheck input[type=radio]').change(function(e){
		console.log('change');
		var valueCheck = $(this).val();
		if(valueCheck == 'email'){
			$('.phoneCheckShow').hide();
			$('.emailCheckShow').show();
		}
		else{
			$('.phoneCheckShow').show();
			$('.emailCheckShow').hide();
		}
	});
	$('#otp_submit').click(function(){
		var otpCode = $(this).closest('.otpcode-form').find('#otp_code').val();
		if(otpCode == ''){
			$(this).closest('.otpcode-form').find('.error').html('Quý khách vui lòng nhập chính xác mã OTP được gửi đến số điện thoại của quý khách');
		}else{
			$(this).closest('.otpcode-form').submit();
		}
	})

							
						

						
							
								
									
									
										  
									
								
								
									0
										
							

	
		
			
		
	
	
		Giỏ hàng
		
			
				
					
					
						
							
						
						
												
					
				   
				
			
			
				
					
					
						
							      
							Hiện chưa có sản phẩm
														
					
					
				
			
			
			
				
					TỔNG TIỀN:
					0₫
				
				
					Xem giỏ hàng
					Thanh toán
				
			
		
	




						

					</value>
      <webElementGuid>6fed3998-ac13-4952-8506-350ec56f4a40</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>id(&quot;site-header&quot;)/div[@class=&quot;header_middle&quot;]/div[@class=&quot;container-fluid&quot;]/div[@class=&quot;flexContainer-header flexAlignCenter rowFlexMargin&quot;]/div[@class=&quot;header-wrap-icon&quot;]</value>
      <webElementGuid>b395f302-02f9-4e0d-9c3b-cf719486218a</webElementGuid>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:idRelative</name>
      <type>Main</type>
      <value>//header[@id='site-header']/div[2]/div/div/div[4]</value>
      <webElementGuid>837d9e64-9a6b-4f38-ba2b-58057a917d00</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Cộng tác viên'])[1]/following::div[1]</value>
      <webElementGuid>86253b07-a3cc-4583-bd13-d9b7687bb5df</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Chính sách thanh toán'])[1]/following::div[1]</value>
      <webElementGuid>97f0de39-f9d0-44af-bbcd-c6d5184dcf84</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//div/div/div[4]</value>
      <webElementGuid>1f6c795a-14da-4668-8203-bd33d85efd5f</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:customAttributes</name>
      <type>Main</type>
      <value>//div[(text() = concat(&quot;
						
							
								
									  
									
										  
									
								
							
							
								
									  
								
								
									Tìm kiếm
									
										
											
											
											
												
											
										
										
											
										
									
								
							
						
						
							
								
									
									
										  
									
								
							


	
		
			
				
					Đăng nhập tài khoản
					Nhập email và mật khẩu của bạn:
				
				
					



					

					
						
						Nhập email hoặc số điện thoại
					
					
						
						
						Mật khẩu						
										
					
					
            Đăng nhập Google
            Đăng nhập Facebook
            
          
					Đăng nhập
					
					
grecaptcha.ready(function() {grecaptcha.execute(&quot; , &quot;'&quot; , &quot;6LdD18MUAAAAAHqKl3Avv8W-tREL6LangePxQLM-&quot; , &quot;'&quot; , &quot;, {action: &quot; , &quot;'&quot; , &quot;submit&quot; , &quot;'&quot; , &quot;}).then(function(token) {document.getElementById(&quot; , &quot;'&quot; , &quot;16330fa71c994363bb3bc902339e86ec&quot; , &quot;'&quot; , &quot;).value = token;});});
					
						Khách hàng mới? 
							Tạo tài khoản
						
						Quên mật khẩu? 
							Khôi phục mật khẩu
						
					
				
			
		
		
			
			
				Khôi phục mật khẩu
			
			
				
					
					Khôi phục bằng email
				
				
					
					Khôi phục bằng số điện thoại
				
			
			

									
					
						
						
							
						
						
							
						
					
					
						
						
						
							
						
						
							
						
					
					
						function On_PhoneAuthRecaptchaCallback(token)
						{
							var frm = $(&quot; , &quot;'&quot; , &quot;#phone_auth_recaptcha&quot; , &quot;'&quot; , &quot;);

							$.ajax({
								type: &quot;POST&quot;,
								url: &quot; , &quot;'&quot; , &quot;/phone_auth/send_verify_code&quot; , &quot;'&quot; , &quot;,
								data: frm.serialize(),
								dataType : &quot;json&quot;,
								success: function(data, textStatus, jqXHR)
								{
									if(data &amp;&amp; data.token)
									{
										if($(&quot; , &quot;'&quot; , &quot;#session_info&quot; , &quot;'&quot; , &quot;).length > 0){
											$(&quot; , &quot;'&quot; , &quot;#session_info&quot; , &quot;'&quot; , &quot;).val(data.token);
											$(&quot; , &quot;'&quot; , &quot;#phone_auth_recaptcha&quot; , &quot;'&quot; , &quot;).hide();
											$(&quot; , &quot;'&quot; , &quot;.otpcode&quot; , &quot;'&quot; , &quot;).show();
										}
									}
								},
								error: function(jqXHR, textStatus, errorThrown)
								{

								}
							});
						}
									
			
			

				
					



					

					
						
						Email
						
					
					Khôi phục

grecaptcha.ready(function() {grecaptcha.execute(&quot; , &quot;'&quot; , &quot;6LdD18MUAAAAAHqKl3Avv8W-tREL6LangePxQLM-&quot; , &quot;'&quot; , &quot;, {action: &quot; , &quot;'&quot; , &quot;submit&quot; , &quot;'&quot; , &quot;}).then(function(token) {document.getElementById(&quot; , &quot;'&quot; , &quot;353e293534434762bdf3734429e2d5f1&quot; , &quot;'&quot; , &quot;).value = token;});});					
						Bạn đã nhớ mật khẩu? 
							Trở về đăng nhập
						
					
				
			
			
		
	


	.phoneCheckShow {
		display: none;
	}
	.otpcode{
		display: none;
	}



	$(&quot; , &quot;'&quot; , &quot;.optionCheck input[type=radio]&quot; , &quot;'&quot; , &quot;).change(function(e){
		console.log(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;);
		var valueCheck = $(this).val();
		if(valueCheck == &quot; , &quot;'&quot; , &quot;email&quot; , &quot;'&quot; , &quot;){
			$(&quot; , &quot;'&quot; , &quot;.phoneCheckShow&quot; , &quot;'&quot; , &quot;).hide();
			$(&quot; , &quot;'&quot; , &quot;.emailCheckShow&quot; , &quot;'&quot; , &quot;).show();
		}
		else{
			$(&quot; , &quot;'&quot; , &quot;.phoneCheckShow&quot; , &quot;'&quot; , &quot;).show();
			$(&quot; , &quot;'&quot; , &quot;.emailCheckShow&quot; , &quot;'&quot; , &quot;).hide();
		}
	});
	$(&quot; , &quot;'&quot; , &quot;#otp_submit&quot; , &quot;'&quot; , &quot;).click(function(){
		var otpCode = $(this).closest(&quot; , &quot;'&quot; , &quot;.otpcode-form&quot; , &quot;'&quot; , &quot;).find(&quot; , &quot;'&quot; , &quot;#otp_code&quot; , &quot;'&quot; , &quot;).val();
		if(otpCode == &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;){
			$(this).closest(&quot; , &quot;'&quot; , &quot;.otpcode-form&quot; , &quot;'&quot; , &quot;).find(&quot; , &quot;'&quot; , &quot;.error&quot; , &quot;'&quot; , &quot;).html(&quot; , &quot;'&quot; , &quot;Quý khách vui lòng nhập chính xác mã OTP được gửi đến số điện thoại của quý khách&quot; , &quot;'&quot; , &quot;);
		}else{
			$(this).closest(&quot; , &quot;'&quot; , &quot;.otpcode-form&quot; , &quot;'&quot; , &quot;).submit();
		}
	})

							
						

						
							
								
									
									
										  
									
								
								
									0
										
							

	
		
			
		
	
	
		Giỏ hàng
		
			
				
					
					
						
							
						
						
												
					
				   
				
			
			
				
					
					
						
							      
							Hiện chưa có sản phẩm
														
					
					
				
			
			
			
				
					TỔNG TIỀN:
					0₫
				
				
					Xem giỏ hàng
					Thanh toán
				
			
		
	




						

					&quot;) or . = concat(&quot;
						
							
								
									  
									
										  
									
								
							
							
								
									  
								
								
									Tìm kiếm
									
										
											
											
											
												
											
										
										
											
										
									
								
							
						
						
							
								
									
									
										  
									
								
							


	
		
			
				
					Đăng nhập tài khoản
					Nhập email và mật khẩu của bạn:
				
				
					



					

					
						
						Nhập email hoặc số điện thoại
					
					
						
						
						Mật khẩu						
										
					
					
            Đăng nhập Google
            Đăng nhập Facebook
            
          
					Đăng nhập
					
					
grecaptcha.ready(function() {grecaptcha.execute(&quot; , &quot;'&quot; , &quot;6LdD18MUAAAAAHqKl3Avv8W-tREL6LangePxQLM-&quot; , &quot;'&quot; , &quot;, {action: &quot; , &quot;'&quot; , &quot;submit&quot; , &quot;'&quot; , &quot;}).then(function(token) {document.getElementById(&quot; , &quot;'&quot; , &quot;16330fa71c994363bb3bc902339e86ec&quot; , &quot;'&quot; , &quot;).value = token;});});
					
						Khách hàng mới? 
							Tạo tài khoản
						
						Quên mật khẩu? 
							Khôi phục mật khẩu
						
					
				
			
		
		
			
			
				Khôi phục mật khẩu
			
			
				
					
					Khôi phục bằng email
				
				
					
					Khôi phục bằng số điện thoại
				
			
			

									
					
						
						
							
						
						
							
						
					
					
						
						
						
							
						
						
							
						
					
					
						function On_PhoneAuthRecaptchaCallback(token)
						{
							var frm = $(&quot; , &quot;'&quot; , &quot;#phone_auth_recaptcha&quot; , &quot;'&quot; , &quot;);

							$.ajax({
								type: &quot;POST&quot;,
								url: &quot; , &quot;'&quot; , &quot;/phone_auth/send_verify_code&quot; , &quot;'&quot; , &quot;,
								data: frm.serialize(),
								dataType : &quot;json&quot;,
								success: function(data, textStatus, jqXHR)
								{
									if(data &amp;&amp; data.token)
									{
										if($(&quot; , &quot;'&quot; , &quot;#session_info&quot; , &quot;'&quot; , &quot;).length > 0){
											$(&quot; , &quot;'&quot; , &quot;#session_info&quot; , &quot;'&quot; , &quot;).val(data.token);
											$(&quot; , &quot;'&quot; , &quot;#phone_auth_recaptcha&quot; , &quot;'&quot; , &quot;).hide();
											$(&quot; , &quot;'&quot; , &quot;.otpcode&quot; , &quot;'&quot; , &quot;).show();
										}
									}
								},
								error: function(jqXHR, textStatus, errorThrown)
								{

								}
							});
						}
									
			
			

				
					



					

					
						
						Email
						
					
					Khôi phục

grecaptcha.ready(function() {grecaptcha.execute(&quot; , &quot;'&quot; , &quot;6LdD18MUAAAAAHqKl3Avv8W-tREL6LangePxQLM-&quot; , &quot;'&quot; , &quot;, {action: &quot; , &quot;'&quot; , &quot;submit&quot; , &quot;'&quot; , &quot;}).then(function(token) {document.getElementById(&quot; , &quot;'&quot; , &quot;353e293534434762bdf3734429e2d5f1&quot; , &quot;'&quot; , &quot;).value = token;});});					
						Bạn đã nhớ mật khẩu? 
							Trở về đăng nhập
						
					
				
			
			
		
	


	.phoneCheckShow {
		display: none;
	}
	.otpcode{
		display: none;
	}



	$(&quot; , &quot;'&quot; , &quot;.optionCheck input[type=radio]&quot; , &quot;'&quot; , &quot;).change(function(e){
		console.log(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;);
		var valueCheck = $(this).val();
		if(valueCheck == &quot; , &quot;'&quot; , &quot;email&quot; , &quot;'&quot; , &quot;){
			$(&quot; , &quot;'&quot; , &quot;.phoneCheckShow&quot; , &quot;'&quot; , &quot;).hide();
			$(&quot; , &quot;'&quot; , &quot;.emailCheckShow&quot; , &quot;'&quot; , &quot;).show();
		}
		else{
			$(&quot; , &quot;'&quot; , &quot;.phoneCheckShow&quot; , &quot;'&quot; , &quot;).show();
			$(&quot; , &quot;'&quot; , &quot;.emailCheckShow&quot; , &quot;'&quot; , &quot;).hide();
		}
	});
	$(&quot; , &quot;'&quot; , &quot;#otp_submit&quot; , &quot;'&quot; , &quot;).click(function(){
		var otpCode = $(this).closest(&quot; , &quot;'&quot; , &quot;.otpcode-form&quot; , &quot;'&quot; , &quot;).find(&quot; , &quot;'&quot; , &quot;#otp_code&quot; , &quot;'&quot; , &quot;).val();
		if(otpCode == &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;){
			$(this).closest(&quot; , &quot;'&quot; , &quot;.otpcode-form&quot; , &quot;'&quot; , &quot;).find(&quot; , &quot;'&quot; , &quot;.error&quot; , &quot;'&quot; , &quot;).html(&quot; , &quot;'&quot; , &quot;Quý khách vui lòng nhập chính xác mã OTP được gửi đến số điện thoại của quý khách&quot; , &quot;'&quot; , &quot;);
		}else{
			$(this).closest(&quot; , &quot;'&quot; , &quot;.otpcode-form&quot; , &quot;'&quot; , &quot;).submit();
		}
	})

							
						

						
							
								
									
									
										  
									
								
								
									0
										
							

	
		
			
		
	
	
		Giỏ hàng
		
			
				
					
					
						
							
						
						
												
					
				   
				
			
			
				
					
					
						
							      
							Hiện chưa có sản phẩm
														
					
					
				
			
			
			
				
					TỔNG TIỀN:
					0₫
				
				
					Xem giỏ hàng
					Thanh toán
				
			
		
	




						

					&quot;))]</value>
      <webElementGuid>b122ac7c-9c1e-4d64-b40e-7319260b1442</webElementGuid>
   </webElementXpaths>
</WebElementEntity>
