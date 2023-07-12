import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

WebUI.openBrowser('')

WebUI.navigateToUrl('https://bio-ngon.com/products/cai-bo-xoi-goi-250g')

WebUI.click(findTestObject('thanhToan/Page_Ci b xi (Gi 250g)  Bio Ngon/button_Thm vo gi'))

WebUI.click(findTestObject('Object Repository/thanhToan/Page_Tt c sn phm  Bio Ngon/span_Tr v ng nhp_icon-box icon-cart cart-menu'))

WebUI.click(findTestObject('Object Repository/thanhToan/Page_Tt c sn phm  Bio Ngon/a_Thanh ton'))

WebUI.setText(findTestObject('Object Repository/thanhToan/Page_Bio Ngon - Thanh ton n hng/input_H v tn_billing_addressfull_name'), 
    name)

WebUI.setText(findTestObject('Object Repository/thanhToan/Page_Bio Ngon - Thanh ton n hng/input_Email_checkout_useremail'), 
    email)

WebUI.setText(findTestObject('Object Repository/thanhToan/Page_Bio Ngon - Thanh ton n hng/input_S in thoi_billing_addressphone'), 
    sdt)

WebUI.click(findTestObject('Object Repository/thanhToan/Page_Bio Ngon - Thanh ton n hng/input_M bu chnh_customer_pick_at_location'))

WebUI.click(findTestObject('Object Repository/thanhToan/Page_Bio Ngon - Thanh ton n hng/button_Tip tc n phng thc thanh ton'))

WebUI.click(findTestObject('Object Repository/thanhToan/Page_Bio Ngon - Thanh ton n hng/input_Phng thc thanh ton_payment_method_id'))

WebUI.click(findTestObject('Object Repository/thanhToan/Page_Bio Ngon - Thanh ton n hng/button_Hon tt n hng'))

WebUI.verifyElementPresent(findTestObject('thanhToan/Page_Bio Ngon - n hngH-120234/h2_t hng thnh cng'), 1)

WebUI.closeBrowser()

